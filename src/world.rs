use std::rc::Rc;
use std::cell::RefCell;

use bracket_noise::prelude::{
    FastNoise,
    NoiseType,
    FractalType
};
use hashbrown::HashMap;

use crate::blocks::{Chunk, ChunkId, CHUNK_WIDTH, world_coord_to_chunk_id};
use crate::physics::PhysicsObject;
use crate::player::Player;
use crate::entity::Entity;
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
    entities: Vec< Rc< RefCell<dyn Entity> > >,
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
            entities: Vec::new(),
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

    pub fn check_col<O: PhysicsObject>(&self, obj: &O){
        let pd = obj.get_data();
        let c_id = world_coord_to_chunk_id(pd.position.x, pd.position.z);
        let c_x = pd.position.x * INVERSE_VOXEL_WIDTH - c_id.x as f32 * CHUNK_WIDTH as f32;
        let c_z = pd.position.z * INVERSE_VOXEL_WIDTH - c_id.z as f32 * CHUNK_WIDTH as f32;
        println!("WC: {:?}", pd.position);
        println!("CID: {:?}", c_id);
        println!("CC: {}, {}", c_x, c_z);
    }
}