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
    let [_, r, g, b] = color.to_be_bytes();

    let r = (r as f32 * scale) as u8;
    let g = (g as f32 * scale) as u8;
    let b = (b as f32 * scale) as u8;

    u32::from_be_bytes([0xFF, r, g, b])
}

pub fn add_colors(color1: u32, color2: u32) -> u32 {
    let [_, r1, g1, b1] = color1.to_be_bytes();
    let [_, r2, g2, b2] = color2.to_be_bytes();

    let r = r1.saturating_add(r2);
    let g = g1.saturating_add(g2);
    let b = b1.saturating_add(b2);

    u32::from_be_bytes([0xFF, r, g, b])
}
