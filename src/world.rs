use std::vec;

use bracket_noise::prelude::{
    FastNoise,
    NoiseType,
    FractalType
};
use cgmath::InnerSpace;
use hashbrown::HashMap;

use crate::blocks::{Chunk, ChunkId, CHUNK_WIDTH, world_coord_to_chunk_id};
use crate::physics::PhysicsObject;
use crate::render::voxel::{VoxelMesh, INVERSE_VOXEL_WIDTH};

const WORLD_WIDTH: u32 = 16;
const WORLD_DEPTH: u32 = 16;

fn chunk_loader(width: u32, depth: u32, height_map: &FastNoise, device: &wgpu::Device) -> HashMap<ChunkId, Chunk>{
    let mut chunks: HashMap<ChunkId, Chunk> = HashMap::with_capacity((width*depth) as usize);
    let half_width = width / 2;
    let half_depth = depth / 2;
    for x in 0..width{
        for z in 0..depth{
            let id = ChunkId { x: x as i32- half_width as i32, z: z as i32 - half_depth as i32 };
            chunks.insert(id, Chunk::flat_world(id, height_map, device));
        }
    } 
    chunks
}

pub struct World{
    chunks: HashMap<ChunkId, Chunk>,
    height_map: FastNoise
}

impl World{
    pub fn new(device: &wgpu::Device) -> Self{
        let mut height_map = FastNoise::seeded(10);
        height_map.set_noise_type(NoiseType::PerlinFractal);
        height_map.set_fractal_type(FractalType::FBM);
        height_map.set_fractal_octaves(5);
        height_map.set_fractal_gain(0.6);
        height_map.set_fractal_lacunarity(2.0);
        height_map.set_frequency(2.0);

        Self{
            chunks: chunk_loader(WORLD_WIDTH, WORLD_DEPTH, &height_map, device),
            height_map
        }
    }

    // getters
    pub fn get_chunk_meshes(&self) -> Vec<&VoxelMesh>{
        let mut v = Vec::with_capacity(self.chunks.len());
        v.extend(
            self.chunks.iter().map(|(_, chunk)| &chunk.mesh)
        );
        return v
    }

    // this method assumes all poins and vectors are corrected
    // for block space rather than shader space
    // returns a vector representing a ray that collides with a block
    // or just the original vector
    pub fn cast_ray(&self, position: cgmath::Point3<f32>, vector: cgmath::Vector3<f32>) -> cgmath::Vector3<f32>{
        // figure out how much to move if travelling in negative dir
        fn negative_direction(spot: f32) -> f32 { spot - spot.floor() }
        // figure out how much to move if travelling in position dir
        fn positive_direction(spot: f32) -> f32 { spot.ceil() - spot }
        
        let x_dist_calc = if vector.x.signum() >= 0.0 { positive_direction } else { negative_direction };
        let y_dist_calc = if vector.y.signum() >= 0.0 { positive_direction } else { negative_direction };
        let z_dist_calc = if vector.z.signum() >= 0.0 { positive_direction } else { negative_direction };
        
        // FIX: Avoid divide by 0 error
        // not likely to get exactly -0.0000000001
        let d_x_by_y = vector.x / (vector.y + 0.0000000001);
        let d_z_by_y = vector.z / (vector.y + 0.0000000001);

        loop{
            // find minimum amount needed to reach end of block
            // if the next block is occupied, return the amount moved
            // otherwise, 
            // this may need trig

            // how far the vector can travel 
            let dist_x = x_dist_calc(position.x);
            let adjusted_x = dist_x * d_x_by_y;
            let dist_y = y_dist_calc(position.y);
            let adjusted_y = dist_y; // same since Y is used as base
            let dist_z = z_dist_calc(position.z);
            let adjusted_z = dist_z * d_z_by_y;
            if adjusted_x < adjusted_y && adjusted_x < adjusted_z {
                
            } else if adjusted_y < adjusted_x && adjusted_y < adjusted_z {

            } else {

            }
        }
        todo!()
    }

    pub fn check_col<O: PhysicsObject>(&self, obj: &O){
        let pd = obj.get_data();
        let c_id = world_coord_to_chunk_id(pd.position.x, pd.position.z);
        let c_x = pd.position.x * INVERSE_VOXEL_WIDTH - c_id.x as f32 * CHUNK_WIDTH as f32;
        let c_z = pd.position.z * INVERSE_VOXEL_WIDTH - c_id.z as f32 * CHUNK_WIDTH as f32;
        println!("WC: {:?}", pd.position);
        println!("CID: {:?}", c_id);
        println!("CC: {}, {}", c_x, c_z);
        let chunk = self.chunks.get(&c_id);
        // can't do block checks on unloaded chunk
        if chunk.is_none(){ return; }
        let chunk = chunk.unwrap();
        // basically just cast a ray and see if it gets cut. 
        // if it does, move the object by the amount left after the cut
    }
}