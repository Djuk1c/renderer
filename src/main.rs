use drawing::*;
use std::sync::Mutex;
mod drawing;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const RED: u32 = 0xFF2020FF;
const CLEAR_COLOR: u32 = 0xFF101010;

static PIXELS: Mutex<[u32; WIDTH * HEIGHT]> = Mutex::new([0u32; 160000]);

fn main() {
    let mut pixels = PIXELS.lock().unwrap();
    test_scene(&mut pixels);
    save_to_ppm(*pixels);

    //wasm_get_pixels(173, 0.33);
    return;
}

fn test_scene<const SIZE: usize>(pixels: &mut [u32; SIZE]) {
    pixels.fill(CLEAR_COLOR);
    draw_line(pixels, 50, 50, 300, 400, RED);
    draw_rectangle(pixels, 200, 200, 100, 100, RED, false);
    draw_rectangle(pixels, 400, 50, 100, 100, RED, true);
    draw_circle(pixels, 400, 400, 150, RED, false);
    draw_circle(pixels, 400, 400, 100, RED, true);
    draw_triangle(pixels, 50, 50, 150, 50, 50, 150, RED, true);
    draw_triangle(pixels, 52, 152, 152, 152, 152, 52, RED, false);
}

// WASM
#[no_mangle]
pub extern "C" fn wasm_get_pixels(frame: u32, _delta: f32) -> u32 {
    let mut pixels = PIXELS.lock().unwrap();
    let (speed, scale) = (8.0, 15.0);
    let sdelta = (((frame as f32) / speed).sin() * scale) as i32;
    let cdelta = (((frame as f32) / speed).cos() * scale) as i32;

    pixels.fill(CLEAR_COLOR);
    draw_triangle(
        &mut pixels,
        160 + cdelta,
        160 + cdelta,
        240 + sdelta,
        160 + sdelta,
        160 + cdelta,
        240 + cdelta,
        RED,
        true,
    );
    draw_triangle(
        &mut pixels,
        240 - sdelta,
        240 - sdelta,
        240 - cdelta,
        160 - cdelta,
        160 - sdelta,
        240 - sdelta,
        RED,
        false,
    );
    draw_line(
        &mut pixels,
        325 + -cdelta * 2,
        325,
        325 + sdelta * 2,
        200,
        RED,
    );
    draw_line(
        &mut pixels,
        325 + sdelta * 2,
        325,
        325 + -cdelta * 2,
        200,
        RED,
    );
    draw_circle(&mut pixels, 80, 80, 40 + sdelta, RED, false);
    draw_circle(&mut pixels, 80, 80, 40 + -sdelta / 2, RED, true);
    draw_rectangle(&mut pixels, 60 + sdelta, 300 + cdelta, 60, 60, RED, false);
    draw_rectangle(&mut pixels, 60 - sdelta, 300 - cdelta, 60, 60, RED, true);
    return pixels.as_ptr() as u32;
}
#[no_mangle]
pub extern "C" fn wasm_get_width() -> i32 {
    return WIDTH as i32;
}
#[no_mangle]
pub extern "C" fn wasm_get_height() -> i32 {
    return HEIGHT as i32;
}
