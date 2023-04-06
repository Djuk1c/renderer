use glam::{Vec3, Mat4};

pub struct Camera {
    pub pos: Vec3,
    pub direction: Vec3,
    pub speed: f32,
}
impl Camera {
    pub fn new(pos: Vec3, speed: f32) -> Self {
        Self {
            pos, direction: pos + Vec3::new(0.0, 0.0, -1.0), speed
        }
    }
    pub fn get_view_mat(&self) -> Mat4 {
        return Mat4::look_at_rh(self.pos, self.direction, Vec3::new(0.0, 1.0, 0.0));
    }
    pub fn update(&mut self, move_vec: Vec3) {
        self.pos += move_vec * self.speed;
    }
}
