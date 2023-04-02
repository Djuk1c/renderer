use crate::{HEIGHT, WIDTH};
use std::{collections::hash_map::Entry, collections::HashMap, fs::File, io::Write};
use glam::IVec2;
use std::mem::swap;

fn interpolate_points(i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<i32> {
    let mut values = Vec::<i32>::new();
    if i0 == i1 {
        values.push(d0 as i32);
        return values;
    }
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    for _ in i0 as i32 .. i1 as i32 + 1 {
        values.push(d as i32);
        d += a;
    }
    return values;
}

// For testing
pub fn fill_triangle<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    mut p0: IVec2,
    mut p1: IVec2,
    mut p2: IVec2,
    color: u32,
) {
    // Sort the points so that y0 <= y1 <= y2
    if p1.y < p0.y { swap(&mut p1, &mut p0) }
    if p2.y < p0.y { swap(&mut p2, &mut p0) }
    if p2.y < p1.y { swap(&mut p2, &mut p1) }

    // Compute the x coordinates of the triangle edges
    let mut x01 = interpolate_points(p0.y as f32, p0.x as f32, p1.y as f32, p1.x as f32);
    let mut x12 = interpolate_points(p1.y as f32, p1.x as f32, p2.y as f32, p2.x as f32);
    let x02 = interpolate_points(p0.y as f32, p0.x as f32, p2.y as f32, p2.x as f32);

    // Concatenate the short sides
    x01.pop();
    x01.append(&mut x12);

    // Determine which is left and which is right
    let m = x01.len() / 2;
    if x02[m] < x01[m] {
        for y in p0.y .. p2.y {
            for x in x02[(y - p0.y) as usize] .. x01[(y - p0.y) as usize] {
                pixels[(x + y * WIDTH as i32) as usize] = color;
            }
        }
    } else {
        for y in p0.y .. p2.y {
            for x in x01[(y - p0.y) as usize] .. x02[(y - p0.y) as usize] {
                pixels[(x + y * WIDTH as i32) as usize] = color;
            }
        }
    }
}

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
    let mut raster_data: HashMap<i32, (i32, i32)> = HashMap::new();

    // For each draw point in lines
    // Store the lowest and highest X for each Y
    // Draw horizontal lines from that data
    if fill {
        draw_line(pixels, x1, y1, x2, y2, color, Some(&mut raster_data));
        draw_line(pixels, x1, y1, x3, y3, color, Some(&mut raster_data));
        draw_line(pixels, x3, y3, x2, y2, color, Some(&mut raster_data));

        // Fill the triangle
        for (y, (min_x, max_x)) in raster_data {
            draw_line(pixels, min_x, y, max_x, y, color, None);
        }
    } else {
        draw_line(pixels, x1, y1, x2, y2, color, None);
        draw_line(pixels, x1, y1, x3, y3, color, None);
        draw_line(pixels, x3, y3, x2, y2, color, None);
    }
}

pub fn draw_line<const SIZE: usize>(
    pixels: &mut [u32; SIZE],
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: u32,
    mut raster_data: Option<&mut HashMap<i32, (i32, i32)>>,
) {
    let dx: i32 = i32::abs(x2 - x1);
    let dy: i32 = i32::abs(y2 - y1);
    let sx: i32 = if x1 < x2 { 1 } else { -1 };
    let sy: i32 = if y1 < y2 { 1 } else { -1 };

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = x1;
    let mut current_y: i32 = y1;

    loop {
        if current_x >= WIDTH as i32 || current_y >= HEIGHT as i32 || current_y < 0 || current_x < 0
        {
            return;
        }
        pixels[(current_x + current_y * WIDTH as i32) as usize] = color;

        // Scanline
        // Store min_x and max_y for each Y, so i can later draw hor lines and fill the triangle
        // https://www.youtube.com/watch?v=t7Ztio8cwqM
        if raster_data.is_some() {
            let raster_data = raster_data.as_mut().unwrap();
            match raster_data.entry(current_y) {
                Entry::Occupied(o) => {
                    let cur = o.into_mut();
                    cur.0 = if current_x < cur.0 { current_x } else { cur.0 };
                    cur.1 = if current_x > cur.1 { current_x } else { cur.1 };
                    cur
                }
                Entry::Vacant(v) => v.insert((current_x, current_x)),
            };
        }

        if current_x == x2 && current_y == y2 {
            return;
        }
        let error2: i32 = error;

        if error2 > -dx {
            error -= dy;
            current_x += sx;
        }
        if error2 < dy {
            error += dx;
            current_y += sy;
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
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
    fill: bool,
) {
    for ry in y..y + h {
        if 0 < ry && ry < HEIGHT as i32 {
            for rx in x..x + w {
                if 0 < rx && rx < WIDTH as i32 {
                    if !fill {
                        // TODO: Lots of wasted iterations, perhaps a faster algorithm exists
                        if ry == y || ry == y + h - 1 || rx == x || rx == x + h - 1 {
                            pixels[(rx + ry * WIDTH as i32) as usize] = color;
                        }
                    } else {
                        pixels[(rx + ry * WIDTH as i32) as usize] = color;
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
