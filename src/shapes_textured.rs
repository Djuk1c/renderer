#![allow(dead_code)]
use std::{cmp, mem};
use glam::{IVec2, Vec2, Vec3, Vec3Swizzles, Vec4Swizzles, Vec4};

use crate::canvas::{Canvas, WIDTH, HEIGHT};
use crate::mesh::Triangle;
use crate::utils::{scale_color, add_colors};
use std::{collections::hash_map::Entry, collections::HashMap};

// 0 = min, 1 = max
pub struct RasterData {
    x: (i32, i32),
    tex_min: (f32, f32),
    tex_max: (f32, f32),
    light: (f32, f32),
    depth: (f32, f32),
}
impl RasterData {
    pub fn init(x: i32, tex_x: f32, tex_y: f32, light: f32, depth: f32) -> Self {
        Self {  x: (x, x), tex_min: (tex_x, tex_y), tex_max: (tex_x, tex_y), light: (light, light), depth: (depth, depth) }
    }
}

pub fn draw_triangle_tex(
    canvas: &mut Canvas,
    tri: &Triangle,
    texture: &Vec<u32>,
    tex_size: (u32, u32)
) {
    let (p1, p2, p3, z1, z2, z3) = (
        tri.v[0].pos.xy().as_ivec2(), tri.v[1].pos.xy().as_ivec2(), tri.v[2].pos.xy().as_ivec2(), tri.v[0].pos.z, tri.v[1].pos.z, tri.v[2].pos.z
    );
    let (t1, t2, t3) = (tri.v[0].texture, tri.v[1].texture, tri.v[2].texture);
    let (l1, l2, l3) = (tri.v[0].lit, tri.v[1].lit, tri.v[2].lit);

    let raster_data_size = cmp::max(cmp::max(p1.y, p2.y), p3.y) - cmp::min(cmp::min(p1.y, p2.y), p3.y) + 1;
    let mut raster_data: HashMap<i32, RasterData> = HashMap::with_capacity(raster_data_size as usize);

    draw_line_tex(canvas, p1, p2, z1, z2, t1, t2, l1, l2, texture, tex_size, Some(&mut raster_data));
    draw_line_tex(canvas, p1, p3, z1, z3, t1, t3, l1, l3, texture, tex_size, Some(&mut raster_data));
    draw_line_tex(canvas, p2, p3, z2, z3, t2, t3, l2, l3, texture, tex_size, Some(&mut raster_data));

    // Fill the triangle
    for (y, data) in raster_data {
        draw_line_tex(canvas, IVec2::new(data.x.0, y), IVec2::new(data.x.1, y), data.depth.0, data.depth.1, data.tex_min.into(), data.tex_max.into(), data.light.0, data.light.1, texture, tex_size, None);
    }
}

pub fn draw_line_tex(
    canvas: &mut Canvas,
    p1: IVec2,
    p2: IVec2,
    z1: f32,
    z2: f32,
    t1: Vec2,
    t2: Vec2,
    l1: f32,
    l2: f32,
    texture: &Vec<u32>,
    tex_size: (u32, u32),
    mut raster_data: Option<&mut HashMap<i32, RasterData>>,
) {
    let dx: i32 = i32::abs(p2.x - p1.x);
    let dy: i32 = i32::abs(p2.y - p1.y);
    let sx: i32 = if p1.x < p2.x { 1 } else { -1 };
    let sy: i32 = if p1.y < p2.y { 1 } else { -1 };

    let mut error: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x: i32 = p1.x;
    let mut current_y: i32 = p1.y;

    let length = (p1 - p2).abs().max_element() + 1;

    for i in 0..length {
        let mut tex_step_x: f32 = t1.x;
        let mut tex_step_y: f32 = t1.y;

        // Calculate texture step
        if t1.x > t2.x {
            tex_step_x = t1.x - (i as f32 * ((t2.x - t1.x).abs() / length as f32));
        } else if t2.x > t1.x {
            tex_step_x = t1.x + (i as f32 * ((t2.x - t1.x).abs() / length as f32));
        }
        if t1.y > t2.y {
            tex_step_y = t1.y - (i as f32 * ((t2.y - t1.y).abs() / length as f32));
        } else if t2.y > t1.y {
            tex_step_y = t1.y + (i as f32 * ((t2.y - t1.y).abs() / length as f32));
        } 
        let (tx, ty) = (((tex_size.0 - 1) as f32 * tex_step_x).round() as u32, ((tex_size.1 - 1) as f32 * tex_step_y).round() as u32);
        //println!("tex: {:?} {:?} | i: {} | texstep: {} {} | texcoord: {} {} | length: {}", t1, t2, i, tex_step_x, tex_step_y, tx, ty, length);

        // Calculate light step
        let ni = length - i;
        let d1 = ni as f32 / length as f32;
        let d2 = i as f32 / length as f32;
        let light = (l1 * d1) + (l2 * d2);
        //println!("{} {} {} {} {}", l1, l2, d1, d2, light);

        // Calculate depth step
        let mut depth_step = z1;
        if z1 > z2 {
            depth_step = z1 - (i as f32 * ((z2 - z1).abs() / length as f32));
        } else if z2 > z1 {
            depth_step = z1 + (i as f32 * ((z2 - z1).abs() / length as f32));
        } 
        canvas.put_pixel(current_x, current_y, depth_step, scale_color(texture[(tx + ty * tex_size.0) as usize], (0.1 + light).clamp(0.0, 1.0)));

        if raster_data.is_some() {
            let raster_data = raster_data.as_mut().unwrap();
            match raster_data.entry(current_y) {
                Entry::Occupied(o) => {
                    let cur = o.into_mut();
                    if current_x < cur.x.0 {
                        cur.x.0 = current_x;
                        cur.tex_min = (tex_step_x, tex_step_y);
                        cur.light.0 = light;
                        cur.depth.0 = depth_step;
                    }
                    if current_x > cur.x.1 {
                        cur.x.1 = current_x;
                        cur.tex_max = (tex_step_x, tex_step_y);
                        cur.light.1 = light;
                        cur.depth.1 = depth_step;
                    }
                    cur
                }
                Entry::Vacant(v) => v.insert(RasterData::init(current_x, tex_step_x, tex_step_y, light, depth_step)),
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
