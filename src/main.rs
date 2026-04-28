use std::collections::HashSet;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, Event, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, keyboard::{KeyCode, KeyLocation, PhysicalKey}, window::{CursorGrabMode, Window, WindowBuilder}
};

use crate::rendering::{Camera, create_cube};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod draw_helpers;
mod rendering;
mod physics;

const WIDTH: i32 = draw_helpers::WIDTH;
const HEIGHT: i32 = draw_helpers::HEIGHT;

fn grab_cursor(w: &Window) {
    // Use confined grab by default for continuous movement inside the window.
    if w.set_cursor_grab(CursorGrabMode::Confined).is_err() {
        let _ = w.set_cursor_grab(CursorGrabMode::Locked);
    }
    let _ = w.set_cursor_visible(false);
}

fn release_cursor(w: &Window) {
    let _ = w.set_cursor_grab(CursorGrabMode::None);
    let _ = w.set_cursor_visible(true);
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut window: Option<&'static Window> = None;
    let mut pixels: Option<Pixels> = None;

    let mut mouse_button: MouseButton = MouseButton::Left;
    let mut mouse_state: ElementState = ElementState::Released;
    let mut mouse_position: winit::dpi::PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);
    let mut ignore_next_cursor_move = false;
    let mut pressed_keys: HashSet<PhysicalKey> = HashSet::new();

    let mut camera:Camera = Camera::new();
    //Polygon::new(Vector3::new(-1.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 1.0), [255, 0, 0, 255]);
    create_cube(-2.5, 2.5, 10.0, 5.0, 5.0, -5.0);

    draw_helpers::init();

    event_loop.run(move |event, elwt| {
         match event {
            Event::Resumed => {
                if window.is_none() {
                    elwt.set_control_flow(ControlFlow::Poll); // Never sleep and call the closure ASAP
                    let built_window = WindowBuilder::new().with_title("Phys-3d").with_inner_size(PhysicalSize::new(WIDTH, HEIGHT)).build(elwt).unwrap();

                    let size= built_window.inner_size();
                    let window_ref: &'static Window = Box::leak(Box::new(built_window));
                    let surface = SurfaceTexture::new(size.width, size.height, window_ref);

                    let built_pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface).unwrap();

                    window = Some(window_ref);
                    pixels = Some(built_pixels);

                    //let scale_factor: f64 = window_ref.scale_factor();
                }
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested, ..
            } => {

                if let Some(pixels) = &mut pixels {
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::KeyW)) {
                        camera.add_local_position([0.0, 0.0, 0.2]);
                    }
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::KeyS)) {
                        camera.add_local_position([0.0, 0.0, -0.2]);
                    }
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::KeyA)) {
                        camera.add_local_position([-0.2, 0.0, 0.0]);
                    }
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::KeyD)) {
                        camera.add_local_position([0.2, 0.0, 0.0]);
                    }
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::Space)) {
                        camera.add_global_position([0.0, -0.2, 0.0]);
                    }
                    if pressed_keys.contains(&PhysicalKey::Code(KeyCode::ShiftLeft)) {
                        camera.add_global_position([0.0, 0.2, 0.0]);
                    }

                    let frame = pixels.frame_mut();

                    camera.cast_rays();
                    camera.update_screen();

                    let buffer = &draw_helpers::pixel_buffer.lock().unwrap();
                    for i in 0..frame.len() {
                        frame[i] = buffer[i];
                    }

                    /*
                    for i in 0..frame.len()/4 {
                        frame[i*4] = (((i as f32) / ((frame.len() as f32)/4.0)) * 255.0) as u8; // R
                        frame[(i*4)+1] = (((i as f32) / ((frame.len() as f32)/4.0)) * 255.0) as u8; // G
                        frame[(i*4)+2] = (((i as f32) / ((frame.len() as f32)/4.0)) * 255.0) as u8; // B
                        frame[(i*4)+3] = 255; // A
                    }
                    */

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        elwt.exit();
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput {device_id: _ , state, button }, ..
            } => {
                mouse_button = button;
                mouse_state = state;
                if let Some(w) = window {
                    if mouse_button == MouseButton::Left && mouse_state == ElementState::Pressed {
                        grab_cursor(w);
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { device_id: _, position }, ..
            } => {
                if ignore_next_cursor_move {
                    ignore_next_cursor_move = false;
                    mouse_position = position;
                    return;
                }

                let delta_x = (position.x - mouse_position.x) / 100.0;
                let delta_y = (position.y - mouse_position.y) / 100.0;
                camera.rotate_by(delta_x as f32, -delta_y as f32);

                if let Some(w) = window {
                    let size = w.inner_size();
                    let width = size.width as f64;
                    let height = size.height as f64;
                    let mut wrapped = None;

                    if position.x <= 1.0 {
                        wrapped = Some(PhysicalPosition::new(width - 2.0, position.y.clamp(1.0, height - 2.0)));
                    } else if position.x >= width - 1.0 {
                        wrapped = Some(PhysicalPosition::new(1.0, position.y.clamp(1.0, height - 2.0)));
                    } else if position.y <= 1.0 {
                        wrapped = Some(PhysicalPosition::new(position.x.clamp(1.0, width - 2.0), height - 2.0));
                    } else if position.y >= height - 1.0 {
                        wrapped = Some(PhysicalPosition::new(position.x.clamp(1.0, width - 2.0), 1.0));
                    }

                    if let Some(new_pos) = wrapped {
                        let _ = w.set_cursor_position(new_pos);
                        ignore_next_cursor_move = true;
                        mouse_position = new_pos;
                        return;
                    }
                }

                mouse_position = position;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { device_id: _, event, is_synthetic }, ..
            } => {
                if !is_synthetic {
                    match event.state {
                        ElementState::Pressed => {
                            pressed_keys.insert(event.physical_key);
                            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                                if let Some(w) = window {
                                    release_cursor(w);
                                }
                            }
                        }
                        ElementState::Released => {
                            pressed_keys.remove(&event.physical_key);
                        }
                    }
                }
            }
            Event::AboutToWait => {
                window.expect("Bug - Window should exist").request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => {
                elwt.exit();
            }

            _ => {}
        }
    })?;

    Ok(())
}