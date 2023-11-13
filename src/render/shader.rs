use wgpu::{BindGroup};

use super::texture;
use super::vertex::VertexTrait;

pub trait Uniform{
    fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub struct Shader{
    pipeline: wgpu::RenderPipeline,
    bind_groups: Vec<BindGroup>
}

impl Shader{
    pub fn new<V: VertexTrait>(file_data: &str, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, bind_groups: Vec<wgpu::BindGroup>, bind_group_layouts: &[&wgpu::BindGroupLayout]) -> Self{
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(file_data.into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bind_group_layouts,
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        V::get_desc()
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil:  Some(wgpu::DepthStencilState {
                    format: texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None
            }
        );
        Self { 
            pipeline: render_pipeline,
            bind_groups: bind_groups
        }
    }

    ///
    /// Getters
    /// 

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline{
        &self.pipeline
    }

    pub fn get_bind_groups(&self) -> &Vec<wgpu::BindGroup>{
        &self.bind_groups
    }

    ///
    /// Setters
    /// 
    
    pub fn update_bind_group(&mut self, index: usize, new_bind_group: BindGroup){
        self.bind_groups[index] = new_bind_group;
    }
}