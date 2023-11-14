mod voxel;
mod voxel_mesh;

pub use voxel::*;
pub use voxel_mesh::VoxelMesh;

pub const VOXEL_WIDTH: f32 = 0.25;
// this is stored since it is commonly used
pub const INVERSE_VOXEL_WIDTH: f32 = 1.0 / VOXEL_WIDTH;