pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

//
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}
impl Mat4x4 {
    pub fn new() -> Self {
        Self {
            m: [[0.0f32; 4]; 4],
        }
    }
}

//
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

//
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
}
