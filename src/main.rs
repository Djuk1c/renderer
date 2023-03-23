use drawing::*;
use math::*;
mod drawing;
mod math;

use std::f64::consts::PI;
use std::sync::Mutex;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const RED: u32 = 0xFF2020FF;
const GREEN: u32 = 0xFF20FF20;
const CLEAR_COLOR: u32 = 0xFF101010;

static PIXELS: Mutex<[u32; WIDTH * HEIGHT]> = Mutex::new([0u32; 160000]);

fn main() {
    let mut pixels = PIXELS.lock().unwrap();
    test_scene(&mut pixels);
    save_to_ppm(*pixels);

    //wasm_get_pixels(173, 0.33);
    return;
}

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
    let (speed, scale) = (20.0, 1.3);
    let sdelta = ((frame as f32) / speed).sin() * scale;
    let cdelta = ((frame as f32) / speed).cos() * scale;
    let mut cube = Mesh::new();
    let mut mat_proj = Mat4x4::new();

    // South
    cube.triangles
        .push(Triangle::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0));
    cube.triangles
        .push(Triangle::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0));
    // East
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0));
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0));
    // North
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0));
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0));
    // West
    cube.triangles
        .push(Triangle::new(0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0));
    cube.triangles
        .push(Triangle::new(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0));
    // Top
    cube.triangles
        .push(Triangle::new(0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0));
    cube.triangles
        .push(Triangle::new(0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0));
    // Bottom
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0));
    cube.triangles
        .push(Triangle::new(1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0));

    let f_near = 0.1;
    let f_far = 1000.0;
    let f_fov = 90.0;
    let f_aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let f_fov_rad = (1.0 / (f_fov * 0.5 / 180.0 * PI).tan()) as f32;

    fn multiply_matrix_vector(i: &Vec3, o: &mut Vec3, m: &Mat4x4) {
        o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
        o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
        o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
        let w = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

        if w != 0.0 {
            o.x /= w;
            o.y /= w;
            o.z /= w;
        }
    }

    mat_proj.m[0][0] = f_aspect_ratio * f_fov_rad;
    mat_proj.m[1][1] = f_fov_rad;
    mat_proj.m[2][2] = f_far / (f_far - f_near);
    mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
    mat_proj.m[2][3] = 1.0;
    mat_proj.m[3][3] = 0.0;

    for tri in cube.triangles {
        let mut tri_projected = Triangle::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let tri_translated = Triangle::new(
            tri.pos[0].x - 0.5 + sdelta * cdelta,
            tri.pos[0].y + cdelta,
            tri.pos[0].z + 3.0 + sdelta / 2.0,
            tri.pos[1].x - 0.5 + sdelta * cdelta,
            tri.pos[1].y + cdelta,
            tri.pos[1].z + 3.0 + sdelta / 2.0,
            tri.pos[2].x - 0.5 + sdelta * cdelta,
            tri.pos[2].y + cdelta,
            tri.pos[2].z + 3.0 + sdelta / 2.0,
        );

        multiply_matrix_vector(&tri_translated.pos[0], &mut tri_projected.pos[0], &mat_proj);
        multiply_matrix_vector(&tri_translated.pos[1], &mut tri_projected.pos[1], &mat_proj);
        multiply_matrix_vector(&tri_translated.pos[2], &mut tri_projected.pos[2], &mat_proj);

        // Scale into view
        tri_projected.pos[0].x += 1.0;
        tri_projected.pos[0].y += 1.0;
        tri_projected.pos[1].x += 1.0;
        tri_projected.pos[1].y += 1.0;
        tri_projected.pos[2].x += 1.0;
        tri_projected.pos[2].y += 1.0;
        tri_projected.pos[0].x *= 0.5 * WIDTH as f32;
        tri_projected.pos[0].y *= 0.5 * HEIGHT as f32;
        tri_projected.pos[1].x *= 0.5 * WIDTH as f32;
        tri_projected.pos[1].y *= 0.5 * HEIGHT as f32;
        tri_projected.pos[2].x *= 0.5 * WIDTH as f32;
        tri_projected.pos[2].y *= 0.5 * HEIGHT as f32;

        draw_triangle(
            &mut pixels,
            tri_projected.pos[0].x as i32,
            tri_projected.pos[0].y as i32,
            tri_projected.pos[1].x as i32,
            tri_projected.pos[1].y as i32,
            tri_projected.pos[2].x as i32,
            tri_projected.pos[2].y as i32,
            GREEN,
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
