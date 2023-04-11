use glam::{Vec3, Mat4};

pub struct Camera {
    pub pos: Vec3,
    pub direction: Vec3,
    pub speed: f32,
    pub front: Vec3,
    pub up: Vec3,
}
impl Camera {
    pub fn new(pos: Vec3, speed: f32) -> Self {
        Self {
            pos, 
            direction: pos + Vec3::new(0.0, 0.0, -1.0), 
            speed,
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0)
        }
    }
    pub fn get_view_mat(&self) -> Mat4 {
        return Mat4::look_at_rh(self.pos, self.direction, self.up);
    }
    pub fn move_forward(&mut self) {
        self.pos -= self.front * self.speed;
    }
    pub fn move_backward(&mut self) {
        self.pos += self.front * self.speed;
    }
    pub fn move_right(&mut self) {
        self.pos -= Vec3::normalize(Vec3::cross(self.front, self.up)) * self.speed;
    }
    pub fn move_left(&mut self) {
        self.pos += Vec3::normalize(Vec3::cross(self.front, self.up)) * self.speed;
    }
}
