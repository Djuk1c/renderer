const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const RED: u32 = 0xFF2020FF;
use drawing::*;
mod drawing;

fn main() {
    let mut pixels = [0u32; HEIGHT * WIDTH];
    test_scene(&mut pixels);

    save_to_ppm(pixels);
    return;
}

fn test_scene<const SIZE: usize>(pixels: &mut [u32; SIZE]) {
    pixels.fill(0x00000000);
    draw_line(pixels, 50, 50, 300, 400, RED);
    draw_rectangle(pixels, 200, 200, 100, 100, RED, false);
    draw_rectangle(pixels, 400, 50, 100, 100, RED, true);
    draw_circle(pixels, 400, 400, 150, RED, false);
    draw_circle(pixels, 400, 400, 100, RED, true);
    draw_triangle(pixels, 50, 50, 150, 50, 50, 150, RED, true);
    draw_triangle(pixels, 52, 152, 152, 152, 152, 52, RED, false);
}
