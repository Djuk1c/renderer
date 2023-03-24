use glam::Vec3;

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
}
