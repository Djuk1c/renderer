#[cfg(test)]
use crate::*;

#[test]
// Using SDL to present the pixel array on the screen
// This can most likely be done much faster by accessing the pixel array directly, instead
// of copying it on each draw call
fn render_test() {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::rect::Rect;
    use std::time::Instant;

    let mut pixels = PIXELS.lock().unwrap();
    pixels.fill(CLEAR_COLOR);

    // SDL Init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Render test", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        //.accelerated()
        //.software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // End SDL Init

    let cube = Mesh::cow();
    let fov = 90.0;
    let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let near = 0.1;
    let far = 1000.0;
    let mat_proj = Mat4::perspective_lh(fov_rad, aspect_ratio, near, far);
    let mut frame = 0;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return,
                _ => {}
            }
        }

        // RENDER LOOP
        frame += 1;
        let mut to_render: Vec<(Triangle, u32)> = vec![];
        let start = Instant::now();
        pixels.fill(CLEAR_COLOR);

        for (_i, tri) in cube.triangles.iter().enumerate() {
            // Translate the triangle
            let mat_model = Mat4::from_translation(Vec3::new(0.0, 0.0, -140.0))
                * Mat4::from_rotation_y(frame as f32 / 15.0);
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
        let duration = start.elapsed();
        println!("Time to do triangle calculation: {:?}", duration);

        let start = Instant::now();
        for tri in &to_render {
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
        let duration = start.elapsed();
        println!("Time to fill the pixels: {:?}", duration);

        // Draw on SDL
        let start = Instant::now();
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let offset = y * pitch + x * 3;
                        let pixel = pixels[x + y * WIDTH];
                        buffer[offset] = ((pixel >> (8 * 0)) & 0xFF) as u8;
                        buffer[offset + 1] = ((pixel >> (8 * 1)) & 0xFF) as u8;
                        buffer[offset + 2] = ((pixel >> (8 * 2)) & 0xFF) as u8;
                    }
                }
            })
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(0, 0, WIDTH as u32, HEIGHT as u32)),
            )
            .unwrap();
        canvas.present();
        let duration = start.elapsed();
        println!("Time to present with SDL: {:?}", duration);
    }
}
