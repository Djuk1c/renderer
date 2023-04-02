use glam::Vec3;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
pub struct Triangle {
    pub pos: [Vec3; 3],
}
impl Triangle {
    pub fn new(
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        x3: f32,
        y3: f32,
        z3: f32,
    ) -> Self {
        Self {
            pos: [
                Vec3::new(x1, y1, z1),
                Vec3::new(x2, y2, z2),
                Vec3::new(x3, y3, z3),
            ],
        }
    }
    pub fn new_vec(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            pos: [
                a, b, c
            ],
        }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0)
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

    pub fn cube() -> Self {
        let mut cube = Self::new();

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

        return cube;
    }

    pub fn from_obj(path: &str) -> Self {
        let mut model = Self::new();
        let mut cache: Vec<Vec3> = Vec::new();

        let file = File::open(path).unwrap();
        let iter = io::BufReader::new(file).lines();
        for line in iter {
            let line = line.unwrap();
            if line.chars().nth(0).unwrap() == 'v' {
                let vertex = line
                    .split(" ")
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect::<Vec<_>>();
                cache.push(Vec3 {
                    x: vertex[0],
                    y: vertex[1],
                    z: vertex[2],
                });
            } else if line.chars().nth(0).unwrap() == 'f' {
                let f = line
                    .split(" ")
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>();

                model.triangles.push(Triangle::new(
                    cache[(f[0] - 1) as usize].x,
                    cache[(f[0] - 1) as usize].y,
                    cache[(f[0] - 1) as usize].z,
                    cache[(f[1] - 1) as usize].x,
                    cache[(f[1] - 1) as usize].y,
                    cache[(f[1] - 1) as usize].z,
                    cache[(f[2] - 1) as usize].x,
                    cache[(f[2] - 1) as usize].y,
                    cache[(f[2] - 1) as usize].z,
                ));
                //println!(
                //    "obj.triangles.push(Triangle::new({},{},{},{},{},{},{},{},{}));",
                //    cache[(f[0] - 1) as usize].x,
                //    cache[(f[0] - 1) as usize].y,
                //    cache[(f[0] - 1) as usize].z,
                //    cache[(f[1] - 1) as usize].x,
                //    cache[(f[1] - 1) as usize].y,
                //    cache[(f[1] - 1) as usize].z,
                //    cache[(f[2] - 1) as usize].x,
                //    cache[(f[2] - 1) as usize].y,
                //    cache[(f[2] - 1) as usize].z,
                //);
            }
        }

        return model;
    }
}
