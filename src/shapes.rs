use glam::{IVec2, Vec2};

use crate::canvas::{Canvas, WIDTH, HEIGHT};
use std::{collections::hash_map::Entry, collections::HashMap};

pub fn draw_triangle(
    canvas: &mut Canvas,
    p1: IVec2,
    p2: IVec2,
    p3: IVec2,
    color1: u32,
    color2: u32,
    color3: u32,
    fill: bool,
) {
    let mut raster_data: HashMap<i32, (i32, i32)> = HashMap::new();

    // For each draw point in lines
    // Store the lowest and highest X for each Y
    // Draw horizontal lines from that data
    if fill {
        draw_line(canvas, p1, p2, color1, color2, Some(&mut raster_data));
        draw_line(canvas, p1, p3, color1, color3, Some(&mut raster_data));
        draw_line(canvas, p3, p2, color3, color2, Some(&mut raster_data));

        // Fill the triangle
        for (y, (min_x, max_x)) in raster_data {
            let c1 = canvas.get_pixel(min_x, y);
            let c2 = canvas.get_pixel(max_x, y);
            //canvas.put_pixel(x, y, color);
            draw_line(canvas, IVec2::new(min_x, y), IVec2::new(max_x, y), c1, c2, None);
        }
    } else {
        draw_line(canvas, p1, p2, color1, color2, None);
        draw_line(canvas, p1, p3, color1, color3, None);
        draw_line(canvas, p3, p2, color3, color2, None);
    }
}

pub fn draw_line(
    canvas: &mut Canvas,
    p1: IVec2,
    p2: IVec2,
    color1: u32,
    color2: u32,
    mut raster_data: Option<&mut HashMap<i32, (i32, i32)>>,
) {
    let dx: i32 = i32::abs(p2.x - p1.x);
    let dy: i32 = i32::abs(p2.y - p1.y);
    let sx: i32 = if p1.x < p2.x { 1 } else { -1 };
    let sy: i32 = if p1.y < p2.y { 1 } else { -1 };

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = p1.x;
    let mut current_y: i32 = p1.y;

    let mut points: Vec::<IVec2> = vec![];

    loop {
        points.push(IVec2::new(current_x, current_y));

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

        fn scale_color(color: u32, scale: f32) -> u32 {
            let mut b = ((color >> (8 * 0)) & 0xFF) as u8;
            let mut g = ((color >> (8 * 1)) & 0xFF) as u8;
            let mut r = ((color >> (8 * 2)) & 0xFF) as u8;

            b = (b as f32 * scale) as u8;
            g = (g as f32 * scale) as u8;
            r = (r as f32 * scale) as u8;

            let val = u32::from_be_bytes([0xFF, b, g, r]);
            return val;
        }

        fn add_colors(color1: u32, color2: u32) -> u32 {
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

        if current_x == p2.x && current_y == p2.y {
            for (i, point) in points.iter().enumerate() {
                let n = points.len() as u32;
                let i = i as u32;
                let ni = n - i;
                let d1 = ni as f32 / n as f32;
                let d2 = i as f32 / n as f32;
                let c1 = scale_color(color1, d1);
                let c2 = scale_color(color2, d2);
                let color = add_colors(c1, c2);
                canvas.put_pixel(point.x, point.y, color);
            }
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

#[allow(dead_code)]
// TODO: Use IVec2
pub fn draw_circle<const SIZE: usize>(
    canvas: &mut Canvas,
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
                            canvas.put_pixel(rx, ry, color);
                        }
                    } else {
                        if dx * dx + dy * dy <= r * r {
                            canvas.put_pixel(rx, ry, color);
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
// TODO: Use IVec2
pub fn draw_rectangle<const SIZE: usize>(
    canvas: &mut Canvas,
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
                            canvas.put_pixel(rx, ry, color);
                        }
                    } else {
                        canvas.put_pixel(rx, ry, color);
                    }
                }
            }
        }
    }
}
