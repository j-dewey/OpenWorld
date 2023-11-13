use wgpu::VertexBufferLayout;

use super::vertex::{VertexTrait, TutorialVertex};

// this trait allows different meshes to be put 
// into the window_state.render()
// examples of meshes:
// - VoxelMesh
// - QuadMesh
// - MobMesh
// - HUDMesh
pub trait MeshTrait<V: VertexTrait>{
    fn blank() -> Self;
    // should just be V::get_vertex_desc()
    fn get_vertex_desc<'a>() -> VertexBufferLayout<'a>;
    // returns name of the shader the mesh is made for
    fn get_shader() -> String;
    fn get_indices(&self) -> &wgpu::Buffer;
    fn get_vertices(&self) -> &wgpu::Buffer;
    fn get_num_indices(&self) -> u32;
}