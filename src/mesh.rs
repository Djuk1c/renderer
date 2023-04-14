use glam::Vec3;
use std::fs::File;
use std::io::{self, BufRead};

const COLOR: u32 = 0xFF2020FF;
//const COLOR: u32 = 0xFFB0B0B0;

#[derive(Default, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub color: u32,
}
impl Vertex {
    pub fn new(pos: Vec3, normal: Vec3, color: u32) -> Self {
        Self { pos, normal, color }
    }
}

#[derive(Clone)]
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

        let file = File::open(path).unwrap();
        let iter = io::BufReader::new(file).lines();
        for line in iter {
            let line = line.unwrap();

            if line.chars().nth(0).unwrap() == 'v' &&
                line.chars().nth(1).unwrap() == 'n' {
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
            } else if line.chars().nth(0).unwrap() == 'v' {
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
            } else if line.chars().nth(0).unwrap() == 'f' {
                // FaceIndex//NormalIndex
                let f = line
                    .split([' ', '/'].as_ref())
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>();

                model.triangles.push(Triangle::new(
                    Vertex {
                        pos: (pos[(f[0] - 1) as usize]),
                        normal: (norm[(f[1] - 1) as usize]),
                        color: (COLOR) 
                    },
                    Vertex {
                        pos: (pos[(f[2] - 1) as usize]),
                        normal: (norm[(f[3] - 1) as usize]),
                        color: (COLOR)
                    },
                    Vertex {
                        pos: (pos[(f[4] - 1) as usize]),
                        normal: (norm[(f[5] - 1) as usize]),
                        color: (COLOR) 
                    },
                ));
            }
        }
        return model;
    }
}
