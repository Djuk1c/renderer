use glam::{Vec3, Quat, Mat4};

use crate::mesh::*;

pub struct Model {
    pub mesh: Mesh,
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
    pub texture_index: i32,
}
impl Model {
    pub fn new(path: &str, texture_index: i32) -> Self {
        Self {
            mesh: Mesh::from_obj(path),
            translation: Vec3::ZERO,
            scale: Vec3::splat(1.0),
            rotation: Quat::IDENTITY,
            texture_index
        }
    }
    pub fn get_model_mat(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation);
    }
}
