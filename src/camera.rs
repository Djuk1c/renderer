use glam::{Vec3, Mat4};

pub struct Camera {
    pos: Vec3,
    speed: f32,
    front: Vec3,
    up: Vec3,
    sensitivity: f32,
    yaw: f32,
    pitch: f32,
}
impl Camera {
    pub fn new(pos: Vec3, speed: f32, sensitivity: f32) -> Self {
        Self {
            pos, 
            speed,
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            sensitivity,
            yaw: 270.0,
            pitch: 0.0,
        }
    }
    pub fn get_view_mat(&self) -> Mat4 {
        return Mat4::look_at_rh(self.pos, self.pos + self.front, self.up);
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
    pub fn look(&mut self, x: f32, y: f32) {
        let x_offset = x * self.sensitivity;
        let y_offset = y * self.sensitivity;

        self.yaw -= x_offset;
        self.pitch -= y_offset;
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let direction = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        );
        self.front = direction.normalize();
    }
}
