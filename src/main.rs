use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, Event, MouseButton, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod draw_helpers;
mod rendering;
mod physics;

const WIDTH: i32 = draw_helpers::WIDTH;
const HEIGHT: i32 = draw_helpers::HEIGHT;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut window: Option<&'static Window> = None;
    let mut pixels: Option<Pixels> = None;

    let mut mouse_button: MouseButton = MouseButton::Left;
    let mut mouse_state: ElementState = ElementState::Released;
    let mut mouse_position: winit::dpi::PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0); 

    draw_helpers::init();

    event_loop.run(move |event, elwt| {
         match event {
            Event::Resumed => {
                if window.is_none() {
                    elwt.set_control_flow(ControlFlow::Poll); // Never sleep and call the closure ASAP
                    let built_window = WindowBuilder::new().with_title("Falling Sand").with_inner_size(PhysicalSize::new(WIDTH, HEIGHT)).build(elwt).unwrap();

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
                    let frame = pixels.frame_mut();

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
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. }, ..
            } => {
                // position is a PhysicalPosition<f64>
                mouse_position = position;
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