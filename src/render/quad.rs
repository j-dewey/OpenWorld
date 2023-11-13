use crate::direction::AbsolouteDirection;
use super::vertex::VertexTrait;
use super::mesh::MeshTrait;
use super::voxel::{VoxelVertex, VOXEL_WIDTH};

pub const QUAD_INDICES: [u32; 6] = [0, 2, 1, 1, 2, 3];

// a 4 sided mesh 
pub struct Quad<V: VertexTrait>{
    verts: [V; 4]
}

impl<V: VertexTrait> MeshTrait<V> for Quad<V>{
    fn get_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> { V::get_desc() }
    fn get_shader() -> String { "voxel".into() }
    fn blank() -> Self{
        todo!();
    }
    fn get_indices(&self) -> &wgpu::Buffer {
        // counter clockwise. < tl, bl, tr > < tr, bl, br >
        vec![0, 2, 1, 1, 2, 3];
        unimplemented!()
    }
    fn get_vertices(&self) -> &wgpu::Buffer {
        unimplemented!()
    }
    fn get_num_indices(&self) -> u32 {
        6
    }
}

impl<V: VertexTrait> Quad<V>{
    pub fn new(verts: [V; 4]) -> Self{
        Self{ verts }
    }
}


pub fn south_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::South.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::South.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::South.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::South.into()}
    ]
}

pub fn north_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
        VoxelVertex{ color,
            block_id, 
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::North.into()},
        VoxelVertex{ color,
            block_id,
             position: [pos[0] as f32, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::North.into()},
        VoxelVertex{ color,
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::North.into()},
        VoxelVertex{ color,
            block_id,
            position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::North.into()}
    ]
}

pub fn east_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::East.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::East.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::East.into()},
        VoxelVertex{ color,
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::East.into()}
    ]
}

pub fn west_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
        VoxelVertex{ color,  
            block_id,
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::West.into()},
        VoxelVertex{ color,  
            block_id,
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::West.into()},
        VoxelVertex{ color,  
            block_id,
            position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::West.into()},
        VoxelVertex{ color,  
            block_id,
            position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::West.into()}
    ]
}

pub fn top_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
            VoxelVertex{ color, 
                block_id,
                position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::Up.into()},
            VoxelVertex{ color, 
                block_id,
                position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32], direction: AbsolouteDirection::Up.into()},
            VoxelVertex{ color, 
                block_id,
                position: [pos[0] as f32, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::Up.into()},
            VoxelVertex{ color, 
                block_id,
                position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32 + VOXEL_WIDTH, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::Up.into()}
    ]
}

pub fn bottom_face(pos: [i32; 3], color: [f32; 3], block_id: u32) -> [VoxelVertex; 4]{
    let pos = [
        pos[0] as f32 * VOXEL_WIDTH,
        pos[1] as f32 * VOXEL_WIDTH,
        pos[2] as f32 * VOXEL_WIDTH
    ];
    [
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::Down.into()},
        VoxelVertex{ color,
            block_id,
             position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32 + VOXEL_WIDTH], direction: AbsolouteDirection::Down.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::Down.into()},
        VoxelVertex{ color, 
            block_id,
            position: [pos[0] as f32 + VOXEL_WIDTH, pos[1] as f32, pos[2] as f32], direction: AbsolouteDirection::Down.into()}
    ]
}