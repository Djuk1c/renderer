#![allow(dead_code)]
use std::cmp;
use glam::{IVec2, Vec2};

use crate::canvas::{Canvas, WIDTH, HEIGHT};
use crate::utils::{scale_color, add_colors};
use std::{collections::hash_map::Entry, collections::HashMap};

pub fn draw_triangle_tex(
    canvas: &mut Canvas,
    p1: IVec2,
    p2: IVec2,
    p3: IVec2,
    t1: Vec2,
    t2: Vec2,
    t3: Vec2,
    texture: &Vec<u32>,
    tex_w: u32,
    tex_h: u32,
) {
    let raster_data_size = cmp::max(cmp::max(p1.y, p2.y), p3.y) - cmp::min(cmp::min(p1.y, p2.y), p3.y) + 1;
    let mut raster_data: HashMap<i32, (i32, i32, f32, f32, f32, f32)> = HashMap::with_capacity(raster_data_size as usize);

    draw_line_tex(canvas, p1, p2, t1, t2, texture, tex_w, tex_h, Some(&mut raster_data));
    draw_line_tex(canvas, p1, p3, t1, t3, texture, tex_w, tex_h, Some(&mut raster_data));
    draw_line_tex(canvas, p3, p2, t3, t2, texture, tex_w, tex_h, Some(&mut raster_data));

    // Fill the triangle
    for (y, (min_x, max_x, t1x, t1y, t2x, t2y)) in raster_data {
        let t1 = Vec2::new(t1x, t1y);
        let t2 = Vec2::new(t2x, t2y);
        draw_line_tex(canvas, IVec2::new(min_x, y), IVec2::new(max_x, y), t1, t2, texture, tex_w, tex_h, None);
        //println!("{} {}", t1, t2);
    }
}

pub fn draw_line_tex(
    canvas: &mut Canvas,
    p1: IVec2,
    p2: IVec2,
    t1: Vec2,
    t2: Vec2,
    texture: &Vec<u32>,
    tex_w: u32,
    tex_h: u32,
    mut raster_data: Option<&mut HashMap<i32, (i32, i32, f32, f32, f32, f32)>>,
) {
    let dx: i32 = i32::abs(p2.x - p1.x);
    let dy: i32 = i32::abs(p2.y - p1.y);
    let sx: i32 = if p1.x < p2.x { 1 } else { -1 };
    let sy: i32 = if p1.y < p2.y { 1 } else { -1 };

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = p1.x;
    let mut current_y: i32 = p1.y;

    let length = (p1 - p2).abs().max_element() + 1;
    let length_x = (p1 - p2).abs().x;
    let mut length_y = (p1 - p2).abs().y;

    for _ in 0..length {
        let cx = i32::abs(current_x - p1.x);
        let mut cy = i32::abs(current_y - p1.y);
        let mut tex_step_x: f32 = t1.x;
        let mut tex_step_y: f32 = t1.y;

        if raster_data.is_none() {
            cy = cx;
            length_y = length_x;
        }

        if t1.x > t2.x {
            tex_step_x = t1.x - (cx as f32 * ((t2.x - t1.x).abs() / length_x as f32));
        } else if t2.x > t1.x {
            tex_step_x = t1.x + (cx as f32 * ((t2.x - t1.x).abs() / length_x as f32));
        }
        if t1.y > t2.y {
            tex_step_y = t1.y - (cy as f32 * ((t2.y - t1.y).abs() / length_y as f32));
        } else if t2.y > t1.y {
            tex_step_y = t1.y + (cy as f32 * ((t2.y - t1.y).abs() / length_y as f32));
        } 

        let (tx, ty) = ((tex_w as f32 * tex_step_x) as u32, (tex_h as f32 * tex_step_y) as u32);
        let tx = tx.clamp(0, tex_w - 1);
        let ty = ty.clamp(0, tex_h - 1);
        //let tx = tx % tex_w;
        //let ty = ty % tex_w;
        //println!("tex: {:?} {:?} | cur: {} {} | texstep: {} {} | texcoord: {} {}", t1, t2, cx, cy, tex_step_x, tex_step_y, tx, ty);

        //if raster_data.is_none() {
        //    println!("tex: {:?} {:?} | cur: {} {} | texstep: {} {} | texcoord: {} {} [{}]", t1, t2, cx, cy, tex_step_x, tex_step_y, tx, ty, length);
        //}

        canvas.put_pixel(current_x, current_y, texture[(tx + ty * tex_w) as usize]);

        if raster_data.is_some() {
            let raster_data = raster_data.as_mut().unwrap();
            match raster_data.entry(current_y) {
                Entry::Occupied(o) => {
                    let cur = o.into_mut();
                    if current_x < cur.0 {
                        cur.0 = current_x;
                        cur.2 = tex_step_x;
                        cur.3 = tex_step_y;
                    }
                    if current_x > cur.1 {
                        cur.1 = current_x;
                        cur.4 = tex_step_x;
                        cur.5 = tex_step_y;
                    }
                    cur
                }
                Entry::Vacant(v) => v.insert((current_x, current_x, tex_step_x, tex_step_y, tex_step_x, tex_step_x)),
            };
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
