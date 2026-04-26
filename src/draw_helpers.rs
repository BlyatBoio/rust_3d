use std::sync::{Mutex, MutexGuard, LazyLock};

pub const WIDTH:i32 = 500;
pub const HEIGHT:i32 = 500;

pub static pixel_buffer: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static changed_pixel_indecies: LazyLock<Mutex<Vec<i32>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static mut stroke_color: Color = Color{red:100, green:100, blue:100, alpha:255};
pub static mut fill_color: Color = Color{red:255, green:255, blue:255, alpha:255};
pub static ellipse_draw_mode: Mutex<DrawMode> = Mutex::new(DrawMode::CENTER);
pub static rect_draw_mode: Mutex<DrawMode> = Mutex::new(DrawMode::TOPLEFT);

pub struct Color{
    pub red:u8,
    pub green:u8,
    pub blue:u8,
    pub alpha:u8
}

impl Color{
    pub fn new(red:u8, green:u8, blue:u8, alpha:u8) -> Self{
        Self{red, green, blue, alpha}
    }
}

pub enum DrawMode{
    CENTER,
    TOPLEFT,
    BOTTOMLEFT,
    TOPRIGHT,
    BOTTOMRIGHT
}

pub fn init(){
    for i in 0..4*WIDTH*HEIGHT{
        pixel_buffer.lock().unwrap().push(0);
    }
    print!("{}", pixel_buffer.lock().unwrap().len());
}
pub fn circle(x:u32, y:u32, radius:u32){
    let min_x: u32;
    let max_x: u32;
    let min_y: u32;
    let max_y: u32;
    let center_x: u32;
    let center_y: u32;

    let mut draw_mode = ellipse_draw_mode.lock().unwrap();

    match &mut *draw_mode {
        DrawMode::CENTER => {
            min_x = x-radius/2;
            max_x = x+radius/2;

            min_y = y-radius/2;
            max_y = y+radius/2;

            center_x = x;
            center_y = y;
        }
        DrawMode::TOPLEFT => {
            min_x = x;
            max_x = x+radius;

            min_y = y;
            max_y = y+radius;

            center_x = x+radius/2;
            center_y = y+radius/2;
        }
        DrawMode::BOTTOMLEFT => {
            min_x = x;
            max_x = x+radius;

            min_y = y-radius;
            max_y = y;

            center_x = x+radius/2;
            center_y = y-radius/2;
        }
        DrawMode::TOPRIGHT => {
            min_x = x-radius;
            max_x = x;

            min_y = y;
            max_y = y+radius;

            center_x = x+radius/2;
            center_y = y+radius/2;    
        }
        DrawMode::BOTTOMRIGHT => {
            min_x = x-radius;
            max_x = x;

            min_y = y-radius;
            max_y = y;

            center_x = x+radius/2;
            center_y = y-radius/2;
        }
    }
    let mut buffer = pixel_buffer.lock().unwrap();
    let mut indecies = changed_pixel_indecies.lock().unwrap();
    for i in min_x..max_x{
        for j in min_y..max_y{
            let dist = (((center_x as f32) -(i as f32)).powi(2) + ((center_y as f32)- (j as f32)).powi(2)).sqrt();

            if dist < radius as f32 / 2.0 {
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = fill_color.red;
                        buffer[(base_index+1) as usize] = fill_color.green;
                        buffer[(base_index+2) as usize] = fill_color.blue;
                        buffer[(base_index+3) as usize] = fill_color.alpha};
                }
            }
            else if dist < (radius as f32 + 1.0) / 2.0 {
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = stroke_color.red;
                        buffer[(base_index+1) as usize] = stroke_color.green;
                        buffer[(base_index+2) as usize] = stroke_color.blue;
                        buffer[(base_index+3) as usize] = stroke_color.alpha};
                }
            }
        }
    }
}
pub fn square(x:u32, y:u32, size:u32){
    let start_x: u32;
    let start_y: u32;

    let mut draw_mode = rect_draw_mode.lock().unwrap();

    match &mut *draw_mode {
        DrawMode::CENTER => {
            start_x = x - size/2;
            start_y = y - size/2;
        }
        DrawMode::TOPLEFT => {
            start_x = x;
            start_y = y;
        }
        DrawMode::BOTTOMLEFT => {
            start_x = x;
            start_y = y-size;
        }
        DrawMode::TOPRIGHT => {
            start_x = x-size;
            start_y = y;
        }
        DrawMode::BOTTOMRIGHT => {
            start_x = x-size;
            start_y = y-size;
        }
    }
    let mut buffer = pixel_buffer.lock().unwrap();
    for i in start_x-1..start_x+size+1{
        for j in start_y-1..start_y+size+1{
            if i>start_x&&i<start_x+size&&j>start_y&&j<start_y+size {
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = fill_color.red;
                        buffer[(base_index+1) as usize] = fill_color.green;
                        buffer[(base_index+2) as usize] = fill_color.blue;
                        buffer[(base_index+3) as usize] = fill_color.alpha};
                }
            }
            else{
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = stroke_color.red;
                        buffer[(base_index+1) as usize] = stroke_color.green;
                        buffer[(base_index+2) as usize] = stroke_color.blue;
                        buffer[(base_index+3) as usize] = stroke_color.alpha};
                }
            }
        }
    }
}
pub fn rect(x:u32, y:u32, w:u32, h:u32){
    let start_x: u32;
    let start_y: u32;        
    let mut draw_mode = rect_draw_mode.lock().unwrap();

    match &mut *draw_mode {
        DrawMode::CENTER => {
            start_x = x - w/2;
            start_y = y - h/2;
        }
        DrawMode::TOPLEFT => {
            start_x = x;
            start_y = y;
        }
        DrawMode::BOTTOMLEFT => {
            start_x = x;
            start_y = y-h;
        }
        DrawMode::TOPRIGHT => {
            start_x = x-w;
            start_y = y;
        }
        DrawMode::BOTTOMRIGHT => {
            start_x = x-w;
            start_y = y-h;
        }
    }
    let mut buffer = pixel_buffer.lock().unwrap();
    for i in start_x-1..start_x+w+1{
        for j in start_y-1..start_y+h+1{
            if i>start_x&&i<start_x+w&&j>start_y&&j<start_y+h {
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = fill_color.red;
                        buffer[(base_index+1) as usize] = fill_color.green;
                        buffer[(base_index+2) as usize] = fill_color.blue;
                        buffer[(base_index+3) as usize] = fill_color.alpha};
                }
            }
            else{
                let base_index = (i+(j*WIDTH as u32))*4;
                if !((base_index+3) as usize > buffer.len()){
                    unsafe{
                        buffer[base_index as usize] = stroke_color.red;
                        buffer[(base_index+1) as usize] = stroke_color.green;
                        buffer[(base_index+2) as usize] = stroke_color.blue;
                        buffer[(base_index+3) as usize] = stroke_color.alpha};
                }
            }
        }
    }
}
pub fn line(x1:f32, y1:f32, x2:f32, y2:f32){
    let x_dir: bool = x1<x2;
    let y_rate: i32 = (y1-y2 / x1-x2).ceil() as i32;
    let mut y_value: f32 = y1;

    let mut buffer = pixel_buffer.lock().unwrap();
    for i in 0..(x1-x2) as i32 {
        let base_index: f32;
        if x_dir {base_index = (x1+(i as f32)+(y_value*WIDTH as f32))*4.0;}
        else {base_index = ((x1-(i as f32))+(y_value*WIDTH as f32))*4.0;}

        unsafe {
            buffer[base_index as usize] = stroke_color.red;
            buffer[(base_index as usize)+1] = stroke_color.green;
            buffer[(base_index as usize)+2] = stroke_color.blue;
            buffer[(base_index as usize)+3] = stroke_color.alpha};

        y_value += y_rate as f32;
    }
}
pub fn pixel(mut buffer: MutexGuard<'_, Vec<u8>>, x:u32, y:u32){
    let base_index = ((x+(y*WIDTH as u32))*4 )as usize;

    unsafe {
        buffer[base_index] = fill_color.red;
        buffer[base_index+1] = fill_color.green;
        buffer[base_index+2] = fill_color.blue;
        buffer[base_index+3] = fill_color.alpha};
}
pub fn fill(color:Color){
    unsafe{fill_color = color};
}
pub fn stroke(color:Color){
    unsafe{stroke_color = color};
}
