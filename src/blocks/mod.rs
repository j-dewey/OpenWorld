use bracket_noise::prelude::FastNoise;

use crate::render::voxel::{VoxelMesh, INVERSE_VOXEL_WIDTH};

mod stones;
mod plants;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
// idk y, but the chunks are offset by this value.
// so chunk 0, 0 spawns 8 chunks away from the world 
// coord 0, 0
const WORLD_OFFSET: f32 = 8.0;
pub type ChunkData = [[[usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

// picture it as coordinates on a map
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChunkId{
    pub x: i32,
    pub z: i32
}

// attempts to make this the world failed when I 
// caused a segfault :(
pub struct Chunk{
    // [x][y][z]
    id: ChunkId,
    blocks: ChunkData,
    pub mesh: VoxelMesh
}

impl Chunk{
    pub fn flat_world(id: ChunkId, height_map: &FastNoise, device: &wgpu::Device) -> Self{
        let mut blocks: ChunkData = [[[0usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
    
        for x in 0..CHUNK_WIDTH{
            for z in 0..CHUNK_WIDTH{
                let height_dif = height_map.get_noise(
                    (x as f32 + CHUNK_WIDTH as f32*id.x as f32)/160.0, (z as f32 + CHUNK_WIDTH as f32 * id.z as f32)/100.0
                ) * 50.0;
                // turning height_dif to usize before adding it to 32
                // causes a cool floor effect
                let height = (32.0 + height_dif) as usize;
                println!("dif: {}", height_dif);
                println!("Height: {}", height);
                for y in 0..height{
                    let mut block = 3;
                    if y == height-1{ block = 1; }
                    if y < height-1 && y >= height - 4{ block = 2; }
                    blocks[x][y][z] = block;
                }
            }
        }

        let mesh = VoxelMesh::from_blocks(&id, &blocks, device);

        Self{ id, blocks, mesh }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> usize{
        self.blocks[x][y][z]
    }
}

pub fn world_coord_to_chunk_id(x: f32, z: f32) -> ChunkId{
    let c_x = (x * INVERSE_VOXEL_WIDTH / CHUNK_WIDTH as f32).floor();
    let c_z = (z * INVERSE_VOXEL_WIDTH / CHUNK_WIDTH as f32).floor();
    ChunkId { x: c_x as i32, z: c_z as i32 }
}

pub struct BlockDefintion{
    pub transparency: bool,
    pub color: [f32; 3]
}

pub const AIR: BlockDefintion = BlockDefintion{
    transparency: true,
    color: [0.0, 0.0, 0.0]
};

pub const BLOCK_ARRAY: [BlockDefintion; 16] = [
    AIR,
    plants::GRASS,
    stones::DIRT,
    stones::STONE,
    // padding
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR,
    AIR
];