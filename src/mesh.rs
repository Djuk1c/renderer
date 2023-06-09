use glam::{Vec3, Vec2};
use std::fs::File;
use std::io::{self, BufRead};

const COLOR: u32 = 0xFF2020FF;
//const COLOR: u32 = 0xFFB0B0B0;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub texture: Vec2,
    pub color: u32,
    pub lit: f32,
}
impl Vertex {
    pub fn new(pos: Vec3, normal: Vec3, texture: Vec2, color: u32, lit: f32) -> Self {
        Self { pos, normal, texture, color, lit }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub v: [Vertex; 3],
}
impl Triangle {
    pub fn new(
        p1: Vertex,
        p2: Vertex,
        p3: Vertex,
    ) -> Self {
        Self {
            v: [p1, p2, p3]
        }
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
    // TODO: Rewrite this
    pub fn from_obj(path: &str) -> Self {
        let mut model = Self::new();
        let mut pos = Vec::<Vec3>::new();
        let mut norm = Vec::<Vec3>::new();
        let mut tex = Vec::<Vec2>::new();

        let file = File::open(path).unwrap();
        let iter = io::BufReader::new(file).lines();
        for line in iter {
            let line = line.unwrap();

            if line.starts_with("vn") {
                // Vertex normals
                let normal = line
                    .split(" ")
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect::<Vec<_>>();
                norm.push(Vec3 {
                    x: normal[0],
                    y: normal[1],
                    z: normal[2],
                });
            } else if line.starts_with("vt") {
                // Texture
                let texture = line
                    .split(" ")
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect::<Vec<_>>();
                tex.push(Vec2 {
                    x: texture[0],
                    y: texture[1],
                });
            } else if line.starts_with("v") {
                // Vertex pos
                let vertex = line
                    .split(" ")
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect::<Vec<_>>();
                pos.push(Vec3 {
                    x: vertex[0],
                    y: vertex[1],
                    z: vertex[2],
                });
            } else if line.starts_with("f") {
                // FaceIndex/TextureIndex/NormalIndex
                let f = line
                    .split([' ', '/'].as_ref())
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>();
                //println!("{}", line);

                if tex.len() == 0 {
                    // Non textured mesh
                    model.triangles.push(Triangle::new(
                        Vertex {
                            pos: (pos[(f[0] - 1) as usize]),
                            normal: (norm[(f[1] - 1) as usize]),
                            texture: Vec2::new(-1.0, -1.0),
                            color: (COLOR),
                            lit: 0.0
                        },
                        Vertex {
                            pos: (pos[(f[2] - 1) as usize]),
                            normal: (norm[(f[3] - 1) as usize]),
                            texture: Vec2::new(-1.0, -1.0),
                            color: (COLOR),
                            lit: 0.0
                        },
                        Vertex {
                            pos: (pos[(f[4] - 1) as usize]),
                            normal: (norm[(f[5] - 1) as usize]),
                            texture: Vec2::new(-1.0, -1.0),
                            color: (COLOR),
                            lit: 0.0
                        },
                    ));
                } else {
                    // Textured mesh
                    model.triangles.push(Triangle::new(
                        Vertex {
                            pos: (pos[(f[0] - 1) as usize]),
                            texture: (tex[(f[1] - 1) as usize]),
                            normal: (norm[(f[2] - 1) as usize]),
                            color: (COLOR),
                            lit: 0.0
                        },
                        Vertex {
                            pos: (pos[(f[3] - 1) as usize]),
                            texture: (tex[(f[4] - 1) as usize]),
                            normal: (norm[(f[5] - 1) as usize]),
                            color: (COLOR),
                            lit: 0.0
                        },
                        Vertex {
                            pos: (pos[(f[6] - 1) as usize]),
                            texture: (tex[(f[7] - 1) as usize]),
                            normal: (norm[(f[8] - 1) as usize]),
                            color: (COLOR),
                            lit: 0.0
                        },
                    ));
                }
                //println!("{} {} {}", tex[(f[1] - 1) as usize], tex[(f[4] - 1) as usize], tex[(f[7] - 1) as usize]);
            }
        }
        return model;
    }
}
