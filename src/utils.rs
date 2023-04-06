use crate::canvas::{WIDTH, HEIGHT};
use std::{fs::File, io::Write};
use std::f32::consts::PI;

use glam::Mat4;

#[allow(dead_code)]
pub fn save_to_ppm<const SIZE: usize>(pixels: [u32; SIZE]) {
    let mut file = File::create("output.ppm").unwrap();
    file.write(format!("P6\n{} {} 255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = pixels[x + y * WIDTH];
            let bytes: [u8; 3] = [
                ((pixel >> (8 * 0)) & 0xFF) as u8,
                ((pixel >> (8 * 1)) & 0xFF) as u8,
                ((pixel >> (8 * 2)) & 0xFF) as u8,
            ];
            file.write_all(&bytes).unwrap();
        }
    }
}

pub fn default_mat_proj() -> Mat4 {
    let fov = 90.0;
    let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let near = 0.1;
    let far = 1000.0;
    return Mat4::perspective_rh(fov_rad, aspect_ratio, near, far);
}

pub fn scale_color(color: u32, scale: f32) -> u32 {
    let mut b = ((color >> (8 * 0)) & 0xFF) as u8;
    let mut g = ((color >> (8 * 1)) & 0xFF) as u8;
    let mut r = ((color >> (8 * 2)) & 0xFF) as u8;

    b = (b as f32 * scale) as u8;
    g = (g as f32 * scale) as u8;
    r = (r as f32 * scale) as u8;

    let val = u32::from_be_bytes([0xFF, b, g, r]);
    return val;
}

pub fn add_colors(color1: u32, color2: u32) -> u32 {
    let b1 = ((color1 >> (8 * 0)) & 0xFF) as u8;
    let g1 = ((color1 >> (8 * 1)) & 0xFF) as u8;
    let r1 = ((color1 >> (8 * 2)) & 0xFF) as u8;

    let b2 = ((color2 >> (8 * 0)) & 0xFF) as u8;
    let g2 = ((color2 >> (8 * 1)) & 0xFF) as u8;
    let r2 = ((color2 >> (8 * 2)) & 0xFF) as u8;

    let val = u32::from_be_bytes([
        0xFF,
        (b1 + b2).clamp(0, 255),
        (g1 + g2).clamp(0, 255),
        (r1 + r2).clamp(0, 255),
    ]);

    return val;
}
