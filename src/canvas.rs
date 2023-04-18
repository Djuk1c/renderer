use glam::Vec3;

// Hardcoded so i can avoid vectors, will only have one canvas either way
pub const W_WIDTH: u32 = 1600;  // Screen size
pub const W_HEIGHT: u32 = 900;  // Screen size
pub const WIDTH: usize = (W_WIDTH / 1) as usize;   // Viewport size
pub const HEIGHT: usize = (W_HEIGHT / 1) as usize;  // Viewport size

pub struct Canvas {
    pub pixels: Box<[u32; WIDTH * HEIGHT]>,
    pub depth: Box<[f32; WIDTH * HEIGHT]>,
}
impl Canvas {
    pub fn new() -> Self {
        Self {
            pixels: Box::new([0u32; WIDTH * HEIGHT]),
            depth: Box::new([0f32; WIDTH * HEIGHT])
        }
    }
    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
        self.depth.fill(0.0);
    }
    pub fn put_pixel(&mut self, x: i32, y: i32, z:f32, color: u32) {
        if z > self.get_depth(x, y) {
            self.pixels[(x + y * WIDTH as i32) as usize] = color;
            self.put_depth(x, y, z);
        }
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
        return self.pixels[(x + y * WIDTH as i32) as usize];
    }
    fn put_depth(&mut self, x: i32, y: i32, depth: f32) {
        self.depth[(x + y * WIDTH as i32) as usize] = depth;
    }
    fn get_depth(&self, x: i32, y: i32) -> f32 {
        return self.depth[(x + y * WIDTH as i32) as usize];
    }
    pub fn viewport_to_canvas(pos: &mut Vec3) {
        pos.x += 1.0;
        pos.y += 1.0;
        pos.x *= 0.5 * WIDTH as f32;
        pos.y *= 0.5 * HEIGHT as f32;
    }
}
