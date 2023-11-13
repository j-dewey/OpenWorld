use core::panic;

use bytemuck::{Zeroable, Pod};

use crate::blocks::{ChunkData, ChunkId, CHUNK_WIDTH, BLOCK_ARRAY, CHUNK_HEIGHT};
use crate::render::vertex::VertexTrait;
use crate::render::mesh::MeshTrait;
use crate::render::quad::{
    QUAD_INDICES, north_face, south_face, 
    east_face, west_face, top_face,
    bottom_face
};

pub const VOXEL_WIDTH: f32 = 0.25;

#[derive(Copy, Clone, Debug, Zeroable, Pod)]
#[repr(C)]
pub struct VoxelVertex{
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub block_id: u32,
    pub direction: f32 // can be any AbsolouteDirection
}

impl VertexTrait for VoxelVertex{
    fn get_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // size of VoxelVertex object
            array_stride: std::mem::size_of::<VoxelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                // color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                },
                // id
                wgpu::VertexAttribute{
                    offset: (std::mem::size_of::<[f32; 3]>()*2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32
                },
                // direction
                wgpu::VertexAttribute {
                    // size of position
                    offset: (std::mem::size_of::<[f32; 3]>() * 2 + std::mem::size_of::<u32>()) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32
                }
            ]
        }
    }
}

impl VoxelVertex{
    pub fn new(position: [f32; 3], direction: f32, color: [f32; 3], block_id: u32) -> Self{
        Self { position, direction, block_id, color }
    }
}

#[derive(Debug)]
pub struct Voxel{
    pub(crate) position: [i32; 3],
    pub(crate) vertices: Vec<VoxelVertex>,
    pub(crate) indices: Vec<u32>
}

#[derive(Copy, Clone, Debug)]
pub struct VoxelFaceRenders{
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub top: bool,
    pub bottom: bool
}

impl Voxel{
    pub fn new(pos: [i32; 3], color: [f32; 3], faces: VoxelFaceRenders) -> Self{
        let down = VOXEL_WIDTH;
        let right = VOXEL_WIDTH;
        let back = VOXEL_WIDTH;

        /* (front / back) (top / bottom) (left / right)
        let ftl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32, pos[2] as f32], direction: 0};
        let ftr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32, pos[2] as f32], direction: 1};
        let fbl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32 + down, pos[2] as f32], direction: 2};
        let fbr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32 + down, pos[2] as f32], direction: 3};
        let btl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32, pos[2] as f32 + back], direction: 4};
        let btr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32, pos[2] as f32 + back], direction: 5};
        let bbl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32 + down, pos[2] as f32 + back], direction: 4};
        let bbr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32 + down, pos[2] as f32 + back], direction: 3};
        */

        let south = south_face(pos, color, 1);
        let north = north_face(pos, color, 1);
        let west = west_face(pos, color, 1);
        let east = east_face(pos, color, 1);
        let bottom = bottom_face(pos, color, 1);
        let top = top_face(pos, color, 1);

        let mut vertices: Vec<VoxelVertex> = Vec::with_capacity(16);
        let mut indices: Vec<u32> = Vec::new();
        let mut faces_added = 0;
        if faces.north{
            vertices.extend(north);
            indices.extend(
                QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }
        if faces.south{
            vertices.extend(south);
            indices.extend(
                QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }
        if faces.west{
            vertices.extend(west);
            indices.extend(
                QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }
        if faces.east{
           vertices.extend(east);
           indices.extend(
            QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }
        if faces.top{
            vertices.extend(top);
            indices.extend(
                QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }
        if faces.bottom{
            vertices.extend(bottom);
            indices.extend(
                QUAD_INDICES.iter().map(|i| i + 6*faces_added)
            )
        }

        Self{
            position: pos,
            vertices,
            indices
        }
    }
}

impl MeshTrait<VoxelVertex> for Voxel{
    fn get_shader() -> String { "voxel".into() }
    fn get_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> { VoxelVertex::get_desc() }

    fn blank() -> Self {
        /* 
        let pos = [0,0,0];
        let down = VOXEL_WIDTH;
        let right = VOXEL_WIDTH;
        let back = VOXEL_WIDTH;

        let ftl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32, pos[2] as f32], direction: 0};
        let ftr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32, pos[2] as f32], direction: 0};
        let fbl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32 + down, pos[2] as f32], direction: 0};
        let fbr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32 + down, pos[2] as f32], direction: 0};
        let btl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32, pos[2] as f32 + back], direction: 0};
        let btr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32, pos[2] as f32 + back], direction: 0};
        let bbl = VoxelVertex{ position: [pos[0] as f32, pos[1] as f32 + down, pos[2] as f32 + back], direction: 0};
        let bbr = VoxelVertex{ position: [pos[0] as f32 + right, pos[1] as f32 + down, pos[2] as f32 + back], direction: 0};

        Self{
            position: pos,
            vertices: [ftl, ftr, fbl, fbr, btl, btr, bbl, bbr]
        }
        */
        unimplemented!()
    }

    fn get_indices(&self) -> &wgpu::Buffer {
        vec![
            // face player
            0, 2, 1, 1, 2, 3,
            // back face
            4, 6, 5, 5, 6, 7,
            // left face
            8, 10, 9, 9, 10, 11,
            // right face
            12, 14, 13, 13, 14, 15,
            // top face
            16, 18, 17, 17, 18, 19,
            // bottom face
            20, 22, 21, 21, 22, 23
        ];
        todo!()
    }

    fn get_vertices(&self) -> &wgpu::Buffer {
        // make vertices into a slice then turn slice into vec
        unimplemented!()
    }

    fn get_num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}
