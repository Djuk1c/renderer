use drawing::*;
use mesh::*;
mod drawing;
mod mesh;
mod models;
mod test;

use glam::{Mat4, Vec3, Vec4Swizzles, IVec2};
use std::f32::consts::PI;
use std::sync::Mutex;

// TODO:
// Z Buffer, Color interpolation
// DONE:
// Face culling, Depth sorting, Normals and lighting

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const RED: u32 = 0xFF2020FF;
const GREEN: u32 = 0xFF20FF20;
const WHITE: u32 = 0xFFFFFFFF;
//const CLEAR_COLOR: u32 = 0xFF101010;
const CLEAR_COLOR: u32 = 0xFFD4BC72;

static PIXELS: Mutex<[u32; WIDTH * HEIGHT]> = Mutex::new([0u32; WIDTH * HEIGHT]);

fn main() {
    let mut pixels = PIXELS.lock().unwrap();
    test_scene(&mut pixels);

    save_to_ppm(*pixels);
    return;
}

#[allow(dead_code)]
fn test_scene<const SIZE: usize>(pixels: &mut [u32; SIZE]) {
    pixels.fill(CLEAR_COLOR);

    fill_triangle(pixels, IVec2::new(100, 20), IVec2::new(250, 250), IVec2::new(50, 400), RED);

    //draw_line(pixels, 50, 50, 300, 400, RED, None);
    //draw_rectangle(pixels, 200, 200, 100, 100, RED, false);
    //draw_rectangle(pixels, 400, 50, 100, 100, RED, true);
    //draw_circle(pixels, 400, 400, 150, RED, false);
    //draw_circle(pixels, 400, 400, 100, RED, true);
    //draw_triangle(pixels, 50, 50, 150, 50, 50, 150, RED, true);
    //draw_triangle(pixels, 52, 152, 152, 152, 152, 52, RED, false);
}

// WASM
#[no_mangle]
pub extern "C" fn wasm_cube_test(frame: u32, _delta: f32) -> u32 {
    let mut pixels = PIXELS.lock().unwrap();
    pixels.fill(CLEAR_COLOR);

    // Create this once
    let (speed, scale) = (20.0, 1.3);
    let cube = Mesh::cow();
    let _sdelta = ((frame as f32) / speed).sin() * scale;
    let _cdelta = ((frame as f32) / speed).cos() * scale;

    let fov = 90.0;
    let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let near = 0.1;
    let far = 1000.0;
    let mat_proj = Mat4::perspective_lh(fov_rad, aspect_ratio, near, far);
    //
    let mut to_render: Vec<(Triangle, u32)> = vec![];

    for (_i, tri) in cube.triangles.iter().enumerate() {
        // Translate the triangle
        let mat_model = Mat4::from_translation(Vec3::new(0.0, 0.0, -120.0))
            * Mat4::from_rotation_y(frame as f32 / 20.0);
        let p1 = mat_model * tri.pos[0].extend(1.0);
        let p2 = mat_model * tri.pos[1].extend(1.0);
        let p3 = mat_model * tri.pos[2].extend(1.0);

        // Calculate normals
        let line1 = p2 - p1;
        let line2 = p3 - p1;
        let normal = Vec3::cross(line1.xyz(), line2.xyz()).normalize();

        // Skip if side is invisible (Culling)
        let vcamera = Vec3::new(0.0, 0.0, 0.0);
        if Vec3::dot(normal, p1.xyz() - vcamera) >= 0.0 {
            continue;
        }

        // Shading
        let dir_light = Vec3::new(0.0, 0.0, -1.0).normalize();
        let lit = Vec3::dot(normal, dir_light).abs();
        let c = (RED & !0xFF) | (255.0 * lit * 0.7) as u32;

        // Project it
        let mut p1 = mat_proj.project_point3(p1.xyz());
        let mut p2 = mat_proj.project_point3(p2.xyz());
        let mut p3 = mat_proj.project_point3(p3.xyz());

        // Scale into view
        p1.x += 1.0;
        p1.y += 1.0;
        p1.x *= 0.5 * WIDTH as f32;
        p1.y *= 0.5 * HEIGHT as f32;

        p2.x += 1.0;
        p2.y += 1.0;
        p2.x *= 0.5 * WIDTH as f32;
        p2.y *= 0.5 * HEIGHT as f32;

        p3.x += 1.0;
        p3.y += 1.0;
        p3.x *= 0.5 * WIDTH as f32;
        p3.y *= 0.5 * HEIGHT as f32;

        to_render.push((
            Triangle::new(p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, p3.x, p3.y, p3.z),
            c,
        ));
    }

    // Painters algorithm, depth sorting
    to_render.sort_by(|a, b| {
        let z1 = (a.0.pos[0].z + a.0.pos[1].z + a.0.pos[2].z) / 3.0;
        let z2 = (b.0.pos[0].z + b.0.pos[1].z + b.0.pos[2].z) / 3.0;
        z1.total_cmp(&z2)
    });

    for tri in to_render {
        // Draw
        draw_triangle(
            &mut pixels,
            tri.0.pos[0].x as i32,
            tri.0.pos[0].y as i32,
            tri.0.pos[1].x as i32,
            tri.0.pos[1].y as i32,
            tri.0.pos[2].x as i32,
            tri.0.pos[2].y as i32,
            tri.1,
            true,
        );
    }

    return pixels.as_ptr() as u32;
}

#[no_mangle]
pub extern "C" fn wasm_scene_test(frame: u32, _delta: f32) -> u32 {
    let mut pixels = PIXELS.lock().unwrap();
    let (speed, scale) = (8.0, 20.0);
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
        None,
    );
    draw_line(
        &mut pixels,
        325 + sdelta * 2,
        325,
        325 + -cdelta * 2,
        200,
        RED,
        None,
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
