use glam::Vec3;

// Hardcoded so i can avoid vectors, will only have one canvas either way
pub const WIDTH: usize = 1600;
pub const HEIGHT: usize = 900;

pub struct Canvas {
    pub pixels: [u32; WIDTH * HEIGHT],
    // Z buffer
}
impl Canvas {
    pub fn new() -> Self {
        Self { pixels: [0u32; WIDTH * HEIGHT] }
    }
    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }
    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.pixels[(x + y * WIDTH as i32) as usize] = color;
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
        return self.pixels[(x + y * WIDTH as i32) as usize];
    }
    pub fn viewport_to_canvas(pos: &mut Vec3) {
        pos.x += 1.0;
        pos.y += 1.0;
        pos.x *= 0.5 * WIDTH as f32;
        pos.y *= 0.5 * HEIGHT as f32;
    }
}
