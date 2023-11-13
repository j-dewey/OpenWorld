use wgpu::VertexBufferLayout;
use std::fmt::Debug;

pub trait VertexTrait: Copy + Clone + Debug + bytemuck::Pod + bytemuck::Zeroable{
    fn get_desc<'a>() -> VertexBufferLayout<'a>;
}

// the vertex used at the begining of the wgpu tutorial
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct TutorialVertex{
    pub position: [f32;3],
    pub color: [f32;3]
}

#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct SimpVertex{
    x: f32,
    y: f32,
    z: f32
}

impl TutorialVertex{
    pub fn to_simp(&self) -> SimpVertex{
        return SimpVertex { x: self.position[0], y: self.position[1], z: self.position[2] }
    }
}

impl VertexTrait for TutorialVertex{
    fn get_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TutorialVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

impl VertexTrait for SimpVertex{
    fn get_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SimpVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<f32>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<f32>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32
                },
            ]
        }
    }
}

impl SimpVertex{
    pub fn get_position(&self) -> [f32; 3]{
        [self.x, self.y, self.x]
    }
}