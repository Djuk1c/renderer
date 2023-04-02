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
    use std::time::Instant;
    use std::collections::VecDeque;

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
        .accelerated()
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

    // Model and proj matrix
    //let cube = Mesh::cube();
    let cube = Mesh::from_obj("models/cow.obj");
    let fov = 90.0;
    let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let near = 0.1;
    let far = 1000.0;
    let mat_proj = Mat4::perspective_rh(fov_rad, aspect_ratio, near, far);
    let mut frame = 0;

    // Clipping functions
    fn distance_point_plane(p: &Vec3, plane: &Vec3, plane_n: &Vec3) -> f32 {
        return plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - Vec3::dot(*plane_n, *plane);
        //return Vec3::dot(*plane_n, *p - *plane);
    }

    fn vector_intersect_plane(plane: &Vec3, plane_n: &Vec3, line_start: &Vec3, line_end: &Vec3) -> Vec3
	{
		let plane_d = -Vec3::dot(*plane_n, *plane);
		let ad = Vec3::dot(*line_start, *plane_n);
		let bd = Vec3::dot(*line_end, *plane_n);
		let t = (-plane_d - ad) / (bd - ad);
		let line_start_to_end = *line_end - *line_start;
		let line_to_intersect = line_start_to_end * t;
		return *line_start + line_to_intersect;
	}

    // Thank you @Javidx9
    fn clip_triangle(tri: &Triangle, plane: &Vec3, plane_n: &Vec3) -> Vec::<Triangle> {
        let mut result = Vec::<Triangle>::new();

        let mut inside_points = Vec::<Vec3>::new();
        let mut outside_points = Vec::<Vec3>::new();

        let d0 = distance_point_plane(&tri.pos[0], &plane, &plane_n) > 0.0;
        let d1 = distance_point_plane(&tri.pos[1], &plane, &plane_n) > 0.0;
        let d2 = distance_point_plane(&tri.pos[2], &plane, &plane_n) > 0.0;

        // Checking points
        if d0 {
            inside_points.push(tri.pos[0]);
        } else {
            outside_points.push(tri.pos[0]);
        }
        if d1 {
            inside_points.push(tri.pos[1]);
        } else {
            outside_points.push(tri.pos[1]);
        }
        if d2 {
            inside_points.push(tri.pos[2]);
        } else {
            outside_points.push(tri.pos[2]);
        }

        if inside_points.len() == 3 {
            result.push(tri.clone());
        } else if inside_points.len() == 1 && outside_points.len() == 2 {
            // Triangle should be clipped. As two points lie outside
			// the plane, the triangle simply becomes a smaller triangle
            let mut new = Triangle::zero();

            // The inside point is valid, so keep that...
            new.pos[0] = inside_points[0];

            // but the two new points are at the locations where the 
			// original sides of the triangle (lines) intersect with the plane
            new.pos[1] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[0]);
			new.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[1]);
            result.push(new);
        } else if inside_points.len() == 2 && outside_points.len() == 1 {
            // Triangle should be clipped. As two points lie inside the plane,
			// the clipped triangle becomes a "quad". Fortunately, we can
			// represent a quad with two new triangles
            let mut new_0 = Triangle::zero();
            let mut new_1 = Triangle::zero();

            // The first triangle consists of the two inside points and a new
			// point determined by the location where one side of the triangle
			// intersects with the plane
			new_0.pos[0] = inside_points[0];
			new_0.pos[1] = inside_points[1];
			new_0.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[0]);

            // The second triangle is composed of one of he inside points, a
			// new point determined by the intersection of the other side of the 
			// triangle and the plane, and the newly created point above
            new_1.pos[0] = inside_points[1];
			new_1.pos[1] = new_0.pos[2];
			new_1.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[1], &outside_points[0]);
            result.push(new_0);
            result.push(new_1);
        }

        return result;
    }
    // END Clipping functions

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
        let mut to_render: Vec<(Triangle, u32)> = vec![];
        let start = Instant::now();
        pixels.fill(CLEAR_COLOR);

        frame += 1;
        for (_i, tri) in cube.triangles.iter().enumerate() {
            //let mat_model = Mat4::from_translation(Vec3::new(0.0, -0.5, 2.5))
            //    * Mat4::from_rotation_y(frame as f32 / 200.0);

            // Translate the triangle
            let mat_model = Mat4::from_translation(Vec3::new(0.0, 0.0, 60.0))
                * Mat4::from_rotation_y(frame as f32 / 300.0);
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

            // Flat shading
            let dir_light = Vec3::new(0.0, 0.0, -1.0).normalize();
            let lit = Vec3::dot(normal, dir_light).abs();
            let c = (RED & !0xFF) | (255.0 * lit) as u32;

            // Clipping near plane
            let tri_to_clip = Triangle::new_vec(p1.xyz(), p2.xyz(), p3.xyz());
            let mut clipped = clip_triangle(&tri_to_clip, &Vec3::new(0.0, 0.0, 0.1), &Vec3::new(0.0, 0.0, 1.0));

            for mut tri_c in clipped.iter_mut() {
                // Project it
                tri_c.pos[0] = mat_proj.project_point3(tri_c.pos[0]);
                tri_c.pos[1] = mat_proj.project_point3(tri_c.pos[1]);
                tri_c.pos[2] = mat_proj.project_point3(tri_c.pos[2]);

                // Scale into view
                for mut pos in tri_c.pos.iter_mut() {
                    pos.x += 1.0;
                    pos.y += 1.0;
                    pos.x *= 0.5 * WIDTH as f32;
                    pos.y *= 0.5 * HEIGHT as f32;
                }

                to_render.push((
                    tri_c.clone(),
                    c,
                ));
            }
        }

        // Painters algorithm, depth sorting
        to_render.sort_by(|a, b| {
            let z1 = (a.0.pos[0].z + a.0.pos[1].z + a.0.pos[2].z) / 3.0;
            let z2 = (b.0.pos[0].z + b.0.pos[1].z + b.0.pos[2].z) / 3.0;
            z1.total_cmp(&z2)
        });
        let duration = start.elapsed();
        println!("Time to do {} triangle calculations (Before viewport clip): {:?}", to_render.len(), duration);

        let start = Instant::now();
        for tri in &to_render {

            // Viewport clip
            let mut queue: VecDeque<Triangle> = VecDeque::new();
            queue.push_back(tri.0.clone());
            let mut new_triangles = 1;

            for plane in 0 .. 4 {
                while new_triangles > 0 {
                    let t = queue.pop_front().unwrap();
                    new_triangles -= 1;

                    let clipped = match plane {
                        0 => {
                            clip_triangle(&t, &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0))
                        }
                        1 => {
                            clip_triangle(&t, &Vec3::new(0.0, HEIGHT as f32 - 1.0, 0.0), &Vec3::new(0.0, -1.0, 0.0))
                        }
                        2 => {
                            clip_triangle(&t, &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
                        }
                        _ => {
                            clip_triangle(&t, &Vec3::new(WIDTH as f32 - 1.0, 0.0, 0.0), &Vec3::new(-1.0, 0.0, 0.0))
                        }
                    };
                    for tri in clipped {
                        queue.push_back(tri);
                    }
                }
                new_triangles = queue.len();
            }

            // Draw final triangles
            for clip in queue {
                draw_triangle(
                    &mut pixels,
                    clip.pos[0].x as i32,
                    clip.pos[0].y as i32,
                    clip.pos[1].x as i32,
                    clip.pos[1].y as i32,
                    clip.pos[2].x as i32,
                    clip.pos[2].y as i32,
                    tri.1,
                    true,
                );
                // Wireframe
                draw_triangle(
                    &mut pixels,
                    clip.pos[0].x as i32,
                    clip.pos[0].y as i32,
                    clip.pos[1].x as i32,
                    clip.pos[1].y as i32,
                    clip.pos[2].x as i32,
                    clip.pos[2].y as i32,
                    GREEN,
                    false,
                );
            }
        }
        let duration = start.elapsed();
        println!("Time to fill the pixels: {:?}", duration);

        // Draw on SDL 
        // TODO: Optimize this
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

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        let duration = start.elapsed();
        println!("Time to present with SDL: {:?}", duration);
    }
}
