use glam::Vec3;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Vertex {
    pub pos: Vec3,
    pub normal: [Vec3; 3],
    pub color: u32,
}

#[derive(Clone)]
pub struct Triangle {
    pub pos: [Vec3; 3],
    pub color: u32,
}
impl Triangle {
    pub fn new(
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        c: u32,
    ) -> Self {
        Self {
            pos: [ p1, p2, p3 ],
            color: c,
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
                    cache[(f[0] - 1) as usize],
                    cache[(f[1] - 1) as usize],
                    cache[(f[2] - 1) as usize],
                    0,
                ));
            }
        }
        return model;
    }

    // Debug
    #[allow(dead_code)]
    pub fn cube() -> Self {
        let mut cube = Self::new();
        // South
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0));
        // East
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 1.0), 0));
        // North
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), 0));
        // West
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0));
        // Top
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), 0));
        // Bottom
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 0.0), 0));
        cube.triangles
            .push(Triangle::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0));
        return cube;
    }
}
