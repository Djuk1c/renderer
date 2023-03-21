use crate::{HEIGHT, WIDTH};
use std::{fs::File, io::Write};

pub fn draw_triangle<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: u32,
    fill: bool,
) {
    let a = draw_line(pixels, x1, y1, x2, y2, color);
    let b = draw_line(pixels, x1, y1, x3, y3, color);
    let c = draw_line(pixels, x3, y3, x2, y2, color);

    //println!("{}, {}, {}", a.len(), b.len(), c.len());

    if fill {
        // Might be enough to loop once through the largest and smallest vector
        for (ry1, rx1) in &a {
            for (ry2, rx2) in &b {
                draw_line(pixels, *rx1, *ry1, *rx2, *ry2, color);
            }
        }
        for (ry1, rx1) in &b {
            for (ry2, rx2) in &c {
                draw_line(pixels, *rx1, *ry1, *rx2, *ry2, color);
            }
        }
        for (ry1, rx1) in &a {
            for (ry2, rx2) in &c {
                draw_line(pixels, *rx1, *ry1, *rx2, *ry2, color);
            }
        }
    }
}

pub fn draw_line<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
) -> Vec<(i32, i32)> {
    let dx: i32 = i32::abs(x2 - x1);
    let dy: i32 = i32::abs(y2 - y1);
    let sx: i32 = if x1 < x2 { 1 } else { -1 };
    let sy: i32 = if y1 < y2 { 1 } else { -1 };

    // Records points where Y changes, so i can later draw horizontal lines to fill in the rec
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    let mut raster_info = Vec::new();

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = x1;
    let mut current_y: i32 = y1;
    loop {
        pixels[(current_x + current_y * WIDTH as i32) as usize] = color;

        if current_x == x2 && current_y == y2 {
            return raster_info;
        }
        let error2: i32 = error;

        if error2 > -dx {
            error -= dy;
            current_x += sx;
        }
        if error2 < dy {
            error += dx;
            current_y += sy;
            raster_info.push((current_y, current_x));
        }
    }
}

pub fn draw_circle<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    x: i32,
    y: i32,
    r: i32,
    color: u32,
    fill: bool,
) {
    for ry in y - r..y + r + 1 {
        if 0 < ry && ry < HEIGHT as i32 {
            for rx in x - r..x + r + 1 {
                if 0 < rx && rx < WIDTH as i32 {
                    let dx = rx - x;
                    let dy = ry - y;
                    if !fill {
                        // TODO: Rewrite this with brain
                        let val = (dx * dx + dy * dy) - (r * r);
                        if val > 0 && val <= r * 2 {
                            pixels[(rx + ry * WIDTH as i32) as usize] = color;
                        }
                    } else {
                        if dx * dx + dy * dy <= r * r {
                            pixels[(rx + ry * WIDTH as i32) as usize] = color;
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_rectangle<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: u32,
    fill: bool,
) {
    for ry in y..y + h {
        if 0 < ry && ry < HEIGHT as u32 {
            for rx in x..x + w {
                if 0 < rx && rx < WIDTH as u32 {
                    if !fill {
                        // TODO: Lots of wasted iterations, perhaps a faster algorithm exists
                        if ry == y || ry == y + h - 1 || rx == x || rx == x + h - 1 {
                            pixels[(rx + ry * WIDTH as u32) as usize] = color;
                        }
                    } else {
                        pixels[(rx + ry * WIDTH as u32) as usize] = color;
                    }
                }
            }
        }
    }
}

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
