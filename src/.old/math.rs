use std::f32::consts::PI;

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
    pub fn proj(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let mut zero = Self::new();

        let fov_rad = (1.0 / (fov * 0.5 / 180.0 * PI).tan()) as f32;
        zero.m[0][0] = aspect_ratio * fov_rad;
        zero.m[1][1] = fov_rad;
        zero.m[2][2] = far / (far - near);
        zero.m[3][2] = (-far * near) / (far - near);
        zero.m[2][3] = 1.0;
        zero.m[3][3] = 0.0;

        return zero;
    }
}
pub fn multiply_matrix_vector(i: &Vec3, o: &mut Vec3, m: &Mat4x4) {
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    let w = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
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
