use cgmath;

use crate::{blocks::{Chunk, BLOCK_ARRAY}, render::voxel::VOXEL_WIDTH};

pub const GRAVITY_FORCE: cgmath::Vector3<f32> = cgmath::Vector3{ x: 0.0, y: -1.0, z: 0.0 };

pub trait PhysicsObject{
    fn get_data(&self) -> &PhysicsData;
    fn get_data_mut(&mut self) -> &mut PhysicsData;
    fn update_position(&mut self, new_position: cgmath::Point3<f32>);
}

// 3d version of a hitbox
pub struct PhysicsData{
    pub position: cgmath::Point3<f32>,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    // only rotate perpindicular to y-plane
    pub rotation: cgmath::Rad<f32>,
    // actually just speed, don't worry about it
    pub force: cgmath::Vector3<f32>,
    pub mass: f32
}

impl PhysicsData{
    pub fn r#move(&mut self, movement: cgmath::Vector3<f32>){
        self.position += movement;
    }
}
