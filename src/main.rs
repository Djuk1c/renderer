use std::time::{Duration, Instant};
use glam::{Vec3, Quat, IVec2, Vec2};
use mesh::Vertex;
use model::Model;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use canvas::{Canvas, HEIGHT, WIDTH, W_WIDTH, W_HEIGHT};
use renderer::Renderer;
use shapes::{draw_line, draw_triangle};
use shapes_textured::*;
use utils::{default_mat_proj, load_ppm};
use camera::*;

mod shapes;
mod utils;
mod mesh;
mod canvas;
mod model;
mod renderer;
mod clipping;
mod camera;
mod shapes_textured;

// TODO:
// raster data vector, zbuffer, animations, specular light, color struct, fog, light color
// DONE:
// Normal face culling, Depth sorting, Near and Viewport clipping, lighting, color interpolation,
// smooth shading, camera, fix screen clipping lighting, textures, fix texture bug
// flip horizontal and rotate 180 texture (wrote bash), texture lit, 

fn main() {
    // SDL Init
    let sdl_context = sdl2::init().unwrap();
    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().set_relative_mouse_mode(true);
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Booba", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut sdl_canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let window = sdl_canvas.window_mut();
    window.set_size(W_WIDTH, W_HEIGHT).unwrap();
    let texture_creator = sdl_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // End SDL Init

    let mut canvas = Canvas::new();
    let mut renderer = Renderer::new(default_mat_proj());
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.25, 0.25);
    let mut obj = Model::new("models/arctic_run.obj");
    let (tex, width, height) = load_ppm("textures/arctic.tex");

    obj.translation.z = 24.5;
    obj.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), (165.0_f32).to_radians());
    obj.scale = Vec3::new(10.0, 10.0, 10.0);

    let mut frame = 0;
    let mut last_mouse_x = 0.0;
    let mut last_mouse_y = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::F1 => { renderer.wireframe = !renderer.wireframe }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Process input
        let keys: Vec<_> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        // Move
        if keys.contains(&Keycode::W) {
            camera.move_forward();
        }
        if keys.contains(&Keycode::S) {
            camera.move_backward();
        }
        if keys.contains(&Keycode::A) {
            camera.move_left();
        }
        if keys.contains(&Keycode::D) {
            camera.move_right();
        }
        // Look
        if keys.contains(&Keycode::Up) {
            camera.look(0.0, 2.0);
        }
        if keys.contains(&Keycode::Down) {
            camera.look(0.0, -2.0);
        }
        if keys.contains(&Keycode::Left) {
            camera.look(2.0, 0.0);
        }
        if keys.contains(&Keycode::Right) {
            camera.look(-2.0, 0.0);
        }

        let mouse_x = event_pump.mouse_state().x() as f32;
        let mouse_y = event_pump.mouse_state().y() as f32;
        let change_x = last_mouse_x - mouse_x;
        let change_y = last_mouse_y - mouse_y;
        last_mouse_x = mouse_x;
        last_mouse_y = mouse_y;

        //camera.look(change_x, change_y);
        // END Process input

        let start = Instant::now();
        // -------------------------------- //
        frame += 1;
        obj.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), (frame as f32 / 2.0).to_radians());
        renderer.process_model(&obj, &camera);
        renderer.depth_sort();
        let duration = start.elapsed();
        println!("Process: {:?}", duration);

        let start = Instant::now();
        renderer.draw(&mut canvas, Some(&tex));
        //draw_line(&mut canvas, IVec2::new(420, 20), IVec2::new(311, 102), 0xFFFF0000, 0xFF0000FF, None);
        //draw_triangle_tex(&mut canvas, 
        //    IVec2::new(200, 200), IVec2::new(550, 100), IVec2::new(400, 550),
        //    Vec2::new(0.0, 0.0),
        //    Vec2::new(1.0, 0.0),
        //    Vec2::new(0.0, 1.0),
        //    &tex,
        //    width, height);
        //draw_triangle_tex(&mut canvas, 
        //    IVec2::new(550, 50), IVec2::new(50, 550), IVec2::new(550, 550),
        //    Vec2::new(1.0, 0.0),
        //    Vec2::new(0.0, 1.0),
        //    Vec2::new(1.0, 1.0),
        //    &tex,
        //    width, height);
        let duration = start.elapsed();
        println!("Draw: {:?}", duration);
        // -------------------------------- //

        // Draw on SDL 
        // TODO: Optimize this
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let offset = y * pitch + x * 3;
                        let pixels = &canvas.pixels;
                        let pixel = pixels[x + y * WIDTH];
                        buffer[offset] = ((pixel >> (8 * 0)) & 0xFF) as u8;
                        buffer[offset + 1] = ((pixel >> (8 * 1)) & 0xFF) as u8;
                        buffer[offset + 2] = ((pixel >> (8 * 2)) & 0xFF) as u8;
                    }
                }
            })
            .unwrap();

        sdl_canvas.copy(&texture, None, None).unwrap();
        sdl_canvas.present();
        // END SDL Draw
    }

    return;
}
