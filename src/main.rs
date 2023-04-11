use std::time::{Duration, Instant};
use glam::{Vec3, Quat, IVec2};
use mesh::Vertex;
use model::Model;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use canvas::{Canvas, HEIGHT, WIDTH};
use renderer::Renderer;
use shapes::{draw_line, draw_triangle};
use utils::default_mat_proj;
use camera::*;

mod shapes;
mod utils;
mod mesh;
mod canvas;
mod model;
mod renderer;
mod clipping;
mod camera;

// TODO:
// fix screen clipping lighting fucked, finish camera
// DONE:
// Normal face culling, Depth sorting, Near and Viewport clipping, lighting, color interpolation,
// smooth shading

fn main() {
    // SDL Init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Render test", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut sdl_canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        //.software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = sdl_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // End SDL Init

    let mut canvas = Canvas::new();
    let mut renderer = Renderer::new(default_mat_proj());
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.25);
    let mut cow = Model::new("models/skull_4k.obj");
    //let mut goat = Model::new("models/goat.obj");
    //let mut cube = Model::cube();

    cow.translation.z = 6.0;
    cow.translation.y = -0.5;
    cow.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), (35 as f32).to_radians());
    cow.scale = Vec3::new(0.1, 0.1, 0.1);
    //goat.translation = Vec3::new(18.0, 0.0, 50.0);
    //goat.scale = Vec3::new(0.8, 0.8, 0.8);
    //cube.translation.z = 5.0;
    //cube.translation.y = 0.5;

    // SDL Draw
    let mut frame = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::W => camera.move_forward(),
                        Keycode::S => camera.move_backward(),
                        Keycode::A => camera.move_right(),
                        Keycode::D => camera.move_left(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let start = Instant::now();
        // -------------------------------- //
        frame += 1;
        let foo = (frame as f32 / 20.0).sin();
        //goat.translation.z += foo;
        //cow.translation.z -= foo;
        cow.rotation = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), (frame as f32).to_radians());
        //goat.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, -1.0), (frame as f32).to_radians());
        //cube.rotation = Quat::from_axis_angle(Vec3::new(-0.2, 1.0, 0.0), (frame as f32).to_radians());
        renderer.process_model(&cow, &camera);
        renderer.depth_sort();
        let duration = start.elapsed();
        println!("Process: {:?}", duration);

        let start = Instant::now();
        //renderer.process_model(&goat);
        //renderer.process_model(&cube);
        renderer.draw(&mut canvas);
        let duration = start.elapsed();
        println!("Draw: {:?}", duration);
        println!("{:?}", camera.pos);
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
    }
    // END SDL Draw

    return;
}
