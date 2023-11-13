use wgpu::util::DeviceExt;

use crate::blocks::{ChunkData, ChunkId, BLOCK_ARRAY, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::render::mesh::MeshTrait;
use crate::render::vertex::VertexTrait;
use crate::render::quad::{
    north_face, south_face, east_face,
    west_face, bottom_face, top_face,
    QUAD_INDICES
};

use super::{Voxel, VoxelVertex};

pub struct VoxelMesh{
    verts: Vec<VoxelVertex>,
    indices: Vec<u32>,
    vert_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer
}

impl MeshTrait<VoxelVertex> for VoxelMesh{
    fn blank() -> Self { unimplemented!() }
    fn get_shader() -> String { "voxel".into() }
    fn get_vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> { VoxelVertex::get_desc() }

    // these should be switched to slices later
    fn get_indices(&self) -> &wgpu::Buffer{
        &self.index_buf
    }

    fn get_vertices(&self) -> &wgpu::Buffer{
        &self.vert_buf
    }

    fn get_num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}

impl VoxelMesh{
    pub fn from_voxel(voxel: Voxel, device: &wgpu::Device) -> Self{
        let indices = voxel.indices;
        let verts = voxel.vertices;
        let vert_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts[..]),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        let index_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices[..]),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        Self{ verts, indices, vert_buf, index_buf }
    }

    pub fn from_voxels(voxels: &[Voxel], device: &wgpu::Device) -> Self{
        let mut verts: Vec<VoxelVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut offset = 0usize;
        for vox in voxels{
            verts.extend(vox.vertices.clone());
            let to_add = vox.indices
                .iter()
                .map(|i| i + (4 * offset as u32))
                .collect::<Vec<u32>>();
            indices.extend(to_add);
            offset += 1;
        }
        let vert_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts[..]),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices[..]),
                usage: wgpu::BufferUsages::INDEX
            }
        );
        
        Self { verts, indices, vert_buf, index_buf }
    }

    pub fn from_blocks(chunk_id: &ChunkId, blocks: &ChunkData, device: &wgpu::Device) -> Self{
        let mut x_offset = 0usize;
        let mut y_offset = 0usize;
        let mut z_offset = 0usize;
        let mut verts: Vec<VoxelVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut faces = 0;
        for square in blocks{
            y_offset = 0;
            for col in square{
                z_offset = 0;
                for block in col{
                    // block gives the ID of the block stored in the current position
                    if BLOCK_ARRAY[*block].transparency{ z_offset += 1; continue; }
                    let pos = [
                        chunk_id.x * CHUNK_WIDTH as i32 + x_offset as i32,
                        y_offset as i32,
                        chunk_id.z * CHUNK_WIDTH as i32 + z_offset as i32
                    ];
                    // start making faces
                    let mut faces_on_this_block = 0;
                    if x_offset == 0 || BLOCK_ARRAY[blocks[x_offset-1][y_offset][z_offset]].transparency{
                        verts.extend(west_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }
                    if x_offset == CHUNK_WIDTH-1 || BLOCK_ARRAY[blocks[x_offset+1][y_offset][z_offset]].transparency{
                        verts.extend(east_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }
                    if z_offset == 0 || BLOCK_ARRAY[blocks[x_offset][y_offset][z_offset-1]].transparency{
                        verts.extend(south_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }
                    if z_offset == CHUNK_WIDTH-1 || BLOCK_ARRAY[blocks[x_offset][y_offset][z_offset+1]].transparency{
                        verts.extend(north_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }
                    if y_offset == 0 || BLOCK_ARRAY[blocks[x_offset][y_offset-1][z_offset]].transparency{
                        verts.extend(bottom_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }
                    if y_offset == CHUNK_HEIGHT-1 || BLOCK_ARRAY[blocks[x_offset][y_offset+1][z_offset]].transparency{
                        verts.extend(top_face(pos, BLOCK_ARRAY[*block].color, *block as u32));
                        faces += 1;
                        faces_on_this_block += 1;
                    }

                    for face in 0..faces_on_this_block{
                        let new_indices = QUAD_INDICES.iter()
                            .map(|i| i + ((faces-face) * 4) as u32)
                            .collect::<Vec<u32>>();
                        indices.extend(new_indices);
                    }

                    z_offset += 1;
                }
                y_offset += 1;
            }
            x_offset += 1;
        }

        let vert_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts[..]),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buf = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices[..]),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        Self { verts, indices, vert_buf, index_buf }
    }
}