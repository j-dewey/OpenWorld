use std::rc::Rc;
use std::cell::RefCell;

use bracket_noise::prelude::{
    FastNoise,
    NoiseType,
    FractalType
};

use crate::blocks::{Chunk, ChunkId};
use crate::player::Player;
use crate::entity::Entity;
use crate::render::voxel::VoxelMesh;

fn chunk_loader(width: u32, depth: u32, height_map: &FastNoise, device: &wgpu::Device) -> Vec<Chunk>{
    let mut chunks: Vec<Chunk> = Vec::with_capacity((width*depth) as usize);
    let half_width = width / 2;
    let half_depth = depth / 2;
    for x in 0..width{
        for z in 0..depth{
            chunks.push(Chunk::flat_world(ChunkId { x: x as i32- half_width as i32, z: z as i32 - half_depth as i32 }, height_map, device))
        }
    } 
    chunks
}

pub struct World{
    player: Player,
    entities: Vec< Rc< RefCell<dyn Entity> > >,
    chunks: Vec<Chunk>,
    height_map: FastNoise
}

impl World{
    pub fn new(scrn_width: u32, scrn_height: u32, device: &wgpu::Device) -> Self{
        let mut height_map = FastNoise::seeded(10);
        height_map.set_noise_type(NoiseType::PerlinFractal);
        height_map.set_fractal_type(FractalType::FBM);
        height_map.set_fractal_octaves(5);
        height_map.set_fractal_gain(0.6);
        height_map.set_fractal_lacunarity(2.0);
        height_map.set_frequency(2.0);

        Self{
            player: Player::new(scrn_width, scrn_height),
            entities: Vec::new(),
            chunks: chunk_loader(16, 16, &height_map, device),
            height_map
        }
    }

    // getters
    pub fn get_player_mut(&mut self) -> &mut Player{
        &mut self.player
    }

    pub fn get_chunks(&self) -> Vec<&VoxelMesh>{
        let mut v = Vec::with_capacity(self.chunks.len());
        v.extend(
            self.chunks.iter().map(|chunk| &chunk.mesh)
        );
        return v
    }
}