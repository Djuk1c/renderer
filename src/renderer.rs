use glam::{Mat4, Vec3, Vec4Swizzles, Vec3Swizzles};
use std::collections::VecDeque;

use crate::{mesh::Triangle, model::Model, clipping::clip_triangle, canvas::{Canvas, HEIGHT, WIDTH}, shapes::draw_triangle, utils::scale_color};

pub struct Renderer {
    to_render: Vec<Triangle>,
    mat_proj: Mat4,
}
impl Renderer {
    pub fn new(proj: Mat4) -> Self {
        Self {
            to_render: Vec::<Triangle>::new(),
            mat_proj: proj,
        }
    }
    pub fn process_model(&mut self, model: &Model) {
        let mut to_clip: Vec<Triangle> = vec![];

        for tri in model.mesh.triangles.iter() {
            let mat_model = model.get_model_mat();

            let p1 = mat_model * tri.v[0].pos.extend(1.0);
            let p2 = mat_model * tri.v[1].pos.extend(1.0);
            let p3 = mat_model * tri.v[2].pos.extend(1.0);

            // Calculate plane normal for clipping
            let line1 = p2 - p1;
            let line2 = p3 - p1;
            let normal = Vec3::cross(line1.xyz(), line2.xyz()).normalize();

            // Skip if side is invisible (Culling)
            let vcamera = Vec3::new(0.0, 0.0, 0.0);
            if Vec3::dot(normal, p1.xyz() - vcamera) >= 0.0 {
                continue;
            }

            // Flat shading
            //let dir_light = Vec3::new(0.0, 0.0, -1.0).normalize();
            //let lit = Vec3::dot(normal, dir_light).abs();
            //let c = (RED & !0xFF) | (255.0 * lit) as u32;

            // Clipping near plane
            let mut tri_to_clip = tri.clone();
            tri_to_clip.v[0].pos = p1.xyz();
            tri_to_clip.v[1].pos = p2.xyz();
            tri_to_clip.v[2].pos = p3.xyz();
            let mut clipped = clip_triangle(&tri_to_clip, &Vec3::new(0.0, 0.0, 0.1), &Vec3::new(0.0, 0.0, 1.0));

            for mut tri_c in clipped.iter_mut() {
                // Project it
                tri_c.v[0].pos = self.mat_proj.project_point3(tri_c.v[0].pos);
                tri_c.v[1].pos = self.mat_proj.project_point3(tri_c.v[1].pos);
                tri_c.v[2].pos = self.mat_proj.project_point3(tri_c.v[2].pos);

                //tri_c.v[0].normal = (self.mat_proj * tri_c.v[0].normal.extend(1.0)).xyz();
                //tri_c.v[1].normal = (self.mat_proj * tri_c.v[1].normal.extend(1.0)).xyz();
                //tri_c.v[2].normal = (self.mat_proj * tri_c.v[2].normal.extend(1.0)).xyz();

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
            for clipped in queue {
                self.to_render.push(clipped);
            }
        }
    }
    fn depth_sort(&mut self) {
        // Painters algorithm, depth sorting
        self.to_render.sort_by(|a, b| {
            let z1 = (a.v[0].pos.z + a.v[1].pos.z + a.v[2].pos.z) / 3.0;
            let z2 = (b.v[0].pos.z + b.v[1].pos.z + b.v[2].pos.z) / 3.0;
            z1.total_cmp(&z2)
        });
    }
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear(0xFF020202);
        self.depth_sort();
        let dir_light = Vec3::new(0.0, 0.0, 1.0).normalize();
        for tri in self.to_render.iter() {
            let lit0 = Vec3::dot(tri.v[0].normal, dir_light).abs();
            let lit1 = Vec3::dot(tri.v[1].normal, dir_light).abs();
            let lit2 = Vec3::dot(tri.v[2].normal, dir_light).abs();
            //println!("{:?} {:?} {:?} | {} {} {}", tri.v[0].normal, tri.v[0].normal,tri.v[0].normal, lit0, lit1, lit2);
            draw_triangle(canvas, tri.v[0].pos.xy().as_ivec2(), tri.v[1].pos.xy().as_ivec2(), tri.v[2].pos.xy().as_ivec2(), scale_color(0xFF0202FF, lit0), scale_color(0xFF0202FF, lit1), scale_color(0xFF0202FF, lit2), true);
            //draw_triangle(canvas, tri.v[0].pos.xy().as_ivec2(), tri.v[1].pos.xy().as_ivec2(), tri.v[2].pos.xy().as_ivec2(), 0xFF00FF00, 0xFF00FF00, 0xFF00FF00, false);
        }
        println!("Rendered {} triangles.", self.to_render.len());
        self.to_render.clear();
    }
}
