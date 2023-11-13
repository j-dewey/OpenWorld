use cgmath::{self, Vector3};

use crate::{blocks::Chunk, render::voxel::VOXEL_WIDTH};

pub const GRAVITY_FORCE: Vector3<f32> = Vector3{ x: 0.0, y: -1.0, z: 0.0 };

pub trait PhysicsObject{
    fn get_data(&self) -> &PhysicsData;
    fn get_data_mut(&mut self) -> &mut PhysicsData;
}

// 3d version of a hitbox
pub struct PhysicsData{
    position: cgmath::Point3<f32>,
    width: f32,
    height: f32,
    depth: f32,
    // only rotate perpindicular to y-plane
    rotation: cgmath::Rad<f32>,
    // actually just speed, don't worry about it
    force: cgmath::Vector3<f32>,
    mass: f32
}

const INVERSE_BLOCK_WIDTH: f32 = 1.0 / VOXEL_WIDTH;
fn map_world_coord_to_block_coord(position: &cgmath::Point3<f32>) -> cgmath::Point3<i32>{
    let x = (position.x * INVERSE_BLOCK_WIDTH) as i32;
    let y = (position.y * INVERSE_BLOCK_WIDTH) as i32;
    let z = (position.z * INVERSE_BLOCK_WIDTH) as i32;
    cgmath::Point3 { x, y, z }
}

pub fn fall<O: PhysicsObject>(obj: &mut O, chunk: &Chunk, dt: f32){
    let pd = obj.get_data_mut();
    pd.force += GRAVITY_FORCE * dt;
    let adjusted_coords = map_world_coord_to_block_coord(&pd.position);
    let down_change = pd.force.y * dt;

    // keep going down until a block is hit. If a block is never hit, fall
    for i in 0..down_change as u32{
        
    }
}