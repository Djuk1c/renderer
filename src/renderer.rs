use glam::{Mat4, Vec3, Vec4Swizzles, Vec3Swizzles, Mat3};
use std::collections::{VecDeque, HashMap};

use crate::{mesh::{Triangle, Vertex}, model::Model, clipping::clip_triangle, canvas::{Canvas, HEIGHT, WIDTH}, utils::*, camera::Camera, shapes::*, shapes_textured::draw_triangle_tex};

pub struct Renderer {
    mat_proj: Mat4,
    pub wireframe: bool,
    pub textures: HashMap<i32, (Vec<u32>, u32, u32)>,
    pub tex_num: i32,
}
impl Renderer {
    pub fn new(proj: Mat4) -> Self {
        Self {
            mat_proj: proj,
            wireframe: false,
            textures: HashMap::new(),
            tex_num: 0
        }
    }
    pub fn draw(&mut self, model: &Model, camera: &Camera, canvas: &mut Canvas) {
        //let mut to_clip = Vec::<Triangle>::with_capacity(self.to_render.len());
        let mut to_clip = Vec::<Triangle>::new();
        let mat_model = model.get_model_mat();

        for tri in model.mesh.triangles.iter() {
            // Model transform
            let mut p1 = mat_model * tri.v[0].pos.extend(1.0);
            let mut p2 = mat_model * tri.v[1].pos.extend(1.0);
            let mut p3 = mat_model * tri.v[2].pos.extend(1.0);
            let mod_tran_inv = Mat3::from_mat4(mat_model).inverse().transpose();    // For normals
            let n1 = (mod_tran_inv * tri.v[0].normal).normalize();
            let n2 = (mod_tran_inv * tri.v[1].normal).normalize();
            let n3 = (mod_tran_inv * tri.v[2].normal).normalize();

            // Ambient light
            let ambient_strength = 0.05;

            // Diffuse light
            let dir_light = Vec3::new(0.0, 0.0, -1.0).normalize();
            let lit1 = Vec3::dot(n1, (dir_light - p1.xyz()).normalize()).clamp(0.0, 1.0);
            let lit2 = Vec3::dot(n2, (dir_light - p2.xyz()).normalize()).clamp(0.0, 1.0);
            let lit3 = Vec3::dot(n3, (dir_light - p3.xyz()).normalize()).clamp(0.0, 1.0);

            // Specular light

            // View transform
            let mat_view = camera.get_view_mat();
            p1 = mat_view * p1;
            p2 = mat_view * p2;
            p3 = mat_view * p3;

            // Calculate plane normal for clipping
            let line1 = p2 - p1;
            let line2 = p3 - p1;
            let normal = Vec3::cross(line1.xyz(), line2.xyz()).normalize();

            // Skip if side is invisible (Culling)
            if Vec3::dot(normal, p1.xyz()) >= 0.0 {
                continue;
            }

            // Create tri to clip and project
            let tri_to_clip = Triangle::new(
                Vertex::new(p1.xyz(), n1.xyz(), tri.v[0].texture, scale_color(tri.v[0].color, ambient_strength + lit1), lit1),
                Vertex::new(p2.xyz(), n2.xyz(), tri.v[1].texture, scale_color(tri.v[1].color, ambient_strength + lit2), lit2),
                Vertex::new(p3.xyz(), n3.xyz(), tri.v[2].texture, scale_color(tri.v[2].color, ambient_strength + lit3), lit3),
            );

            // Clip triangle
            let mut clipped = clip_triangle(&tri_to_clip, &Vec3::new(0.0, 0.0, 0.1), &Vec3::new(0.0, 0.0, 1.0));
            for mut tri_c in clipped.iter_mut() {
                // Project it
                tri_c.v[0].pos = self.mat_proj.project_point3(tri_c.v[0].pos);
                tri_c.v[1].pos = self.mat_proj.project_point3(tri_c.v[1].pos);
                tri_c.v[2].pos = self.mat_proj.project_point3(tri_c.v[2].pos);

                // Scale into view
                for vertex in tri_c.v.iter_mut() {
                    Canvas::viewport_to_canvas(&mut vertex.pos);
                }

                to_clip.push(tri_c.clone());
            }
        }

        for tri in &to_clip {
            // Screenspace clip
            let mut queue: VecDeque<Triangle> = VecDeque::new();
            queue.push_back(tri.clone());
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
            for tri in queue.iter() {
                if self.wireframe {
                    draw_triangle(
                        canvas,
                        tri.v[0].pos.xy().as_ivec2(),
                        tri.v[1].pos.xy().as_ivec2(),
                        tri.v[2].pos.xy().as_ivec2(),
                        0xFF00FF00, 0xFF00FF00, 0xFF00FF00, false);
                }
                if model.texture_index != -1 {
                    let texture = self.textures.get(&model.texture_index).unwrap();
                    draw_triangle_tex(canvas, 
                        tri,
                        &texture.0,
                        texture.1, texture.2
                    );
                } else {
                    draw_triangle(canvas,
                        tri.v[0].pos.xy().as_ivec2(),
                        tri.v[1].pos.xy().as_ivec2(),
                        tri.v[2].pos.xy().as_ivec2(),
                        tri.v[0].color,
                        tri.v[1].color,
                        tri.v[2].color,
                        true);
                }
            }
        }
    }
    pub fn load_texture(&mut self, path: &str) -> i32 {
        let (pixels, width, height) = load_pixels(path);
        self.textures.insert(self.tex_num, (pixels, width, height));
        let cur = self.tex_num;
        self.tex_num += 1;
        return cur; 
    }
    //pub fn depth_sort(&mut self) {
    //    // Painters algorithm, depth sorting
    //    self.to_render.sort_by(|a, b| {
    //        let z1 = (a.v[0].pos.z + a.v[1].pos.z + a.v[2].pos.z) / 3.0;
    //        let z2 = (b.v[0].pos.z + b.v[1].pos.z + b.v[2].pos.z) / 3.0;
    //        z1.total_cmp(&z2)
    //    });
    //}
    //pub fn draw(&mut self, canvas: &mut Canvas, texture: Option<&Vec<u32>>) {
    //    canvas.clear(0xFF020202);
    //    if texture.is_some() {
    //        let texture = texture.unwrap();
    //        for (_, tri) in self.to_render.iter_mut().enumerate() {
    //            draw_triangle_tex(canvas, 
    //                tri,
    //                texture,
    //                1024, 1024
    //            );
    //        }
    //    }
    //    else {
    //        for tri in self.to_render.iter() {
    //            draw_triangle(canvas, tri.v[0].pos.xy().as_ivec2(), tri.v[1].pos.xy().as_ivec2(), tri.v[2].pos.xy().as_ivec2(), tri.v[0].color, tri.v[1].color, tri.v[2].color, true);
    //        }
    //    }

    //    // Wireframe
    //    if self.wireframe {
    //        for tri in self.to_render.iter() {
    //            draw_triangle(canvas, tri.v[0].pos.xy().as_ivec2(), tri.v[1].pos.xy().as_ivec2(), tri.v[2].pos.xy().as_ivec2(), 0xFF00FF00, 0xFF00FF00, 0xFF00FF00, false);
    //        }
    //    }
    //    println!("Rendered {} triangles.", self.to_render.len());
    //    self.to_render.clear();
    //}
}
