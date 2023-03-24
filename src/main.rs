use drawing::*;
use mesh::*;
mod drawing;
mod mesh;

//use glam::{vec4, Mat4, Vec3};
use glam::*;
use std::f32::consts::PI;
use std::sync::Mutex;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const RED: u32 = 0xFF2020FF;
const GREEN: u32 = 0xFF20FF20;
const CLEAR_COLOR: u32 = 0xFF101010;

static PIXELS: Mutex<[u32; WIDTH * HEIGHT]> = Mutex::new([0u32; 160000]);

fn main() {
    //test_scene(&mut pixels);

    wasm_cube_test(173, 0.33);
    save_to_ppm(*PIXELS.lock().unwrap());
    return;
}

#[allow(dead_code)]
fn test_scene<const SIZE: usize>(pixels: &mut [u32; SIZE]) {
    pixels.fill(CLEAR_COLOR);
    draw_line(pixels, 50, 50, 300, 400, RED, None);
    draw_rectangle(pixels, 200, 200, 100, 100, RED, false);
    draw_rectangle(pixels, 400, 50, 100, 100, RED, true);
    draw_circle(pixels, 400, 400, 150, RED, false);
    draw_circle(pixels, 400, 400, 100, RED, true);
    draw_triangle(pixels, 50, 50, 150, 50, 50, 150, RED, true);
    draw_triangle(pixels, 52, 152, 152, 152, 152, 52, RED, false);
}

// WASM
#[no_mangle]
pub extern "C" fn wasm_cube_test(frame: u32, _delta: f32) -> u32 {
    let mut pixels = PIXELS.lock().unwrap();
    pixels.fill(CLEAR_COLOR);

    // Create this once
    let (speed, scale) = (20.0, 1.3);
    let sdelta = ((frame as f32) / speed).sin() * scale;
    let cdelta = ((frame as f32) / speed).cos() * scale;
    let cube = Mesh::cube();

    let fov = 90.0;
    let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let near = 0.1;
    let far = 1000.0;
    let mat_proj = Mat4::perspective_rh_gl(fov_rad, aspect_ratio, near, far);
    //

    for (i, tri) in cube.triangles.iter().enumerate() {
        // Translate the triangle
        let p1 = Vec3::new(tri.pos[0].x, tri.pos[0].y, tri.pos[0].z);
        let p2 = Vec3::new(tri.pos[1].x, tri.pos[1].y, tri.pos[1].z);
        let p3 = Vec3::new(tri.pos[2].x, tri.pos[2].y, tri.pos[2].z);
        let mat_model = Mat4::from_translation(Vec3::new(0.0, -0.5, -5.0))
            * Mat4::from_rotation_z(0.0)
            * Mat4::from_rotation_y(frame as f32 / 20.0);
        let p1 = mat_model * p1.extend(1.0);
        let p2 = mat_model * p2.extend(1.0);
        let p3 = mat_model * p3.extend(1.0);

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

        // Draw
        draw_triangle(
            &mut pixels,
            p1.x as i32,
            p1.y as i32,
            p2.x as i32,
            p2.y as i32,
            p3.x as i32,
            p3.y as i32,
            GREEN / (i + 1) as u32,
            false,
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
