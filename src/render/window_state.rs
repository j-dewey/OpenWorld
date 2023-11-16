use winit::window::Window;
use wgpu;
use hashbrown::HashMap;

use super::{shader::Shader, mesh::MeshTrait, vertex::VertexTrait, texture::Texture, text::TextData};

pub struct WindowState{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    shaders: HashMap<String, Shader>,
    depth_texture: Texture,
    text_data: TextData
}

impl WindowState{
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self{
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // surface is a texture we can render to
        // adapter is actual reference to gpu
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::DEBUG,
            dx12_shader_compiler: Default::default(),
            gles_minor_version: Default::default()
        });
        let surface = unsafe { instance.create_surface(window) }
            .expect("Failed to load surface from window");
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // these boys help make things like buffers and textures and shaders
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // what things we can make with this bad boy
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![]
        };
        // make it match the window
        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth(&device, &config, "depth_texture");
        
        let text_data = TextData::new(config.width, config.height, &device, &queue);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shaders: HashMap::new(),
            depth_texture,
            text_data
        }
    }

    ///
    /// Getters
    /// 

    pub fn get_device_ref(&self) -> &wgpu::Device{
        &self.device
    }

    pub fn get_device_ref_mut(&mut self) -> &wgpu::Device{
        // this exists for cases where a &mut pointer needs device
        &self.device
    }

    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration{
        &self.config
    }

    pub fn get_shader_mut(&mut self, shader: String) -> &mut Shader{
        self.shaders.get_mut(&shader).unwrap()
    }

    ///
    /// Other
    /// 

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        self.text_data.resize(new_size.width, new_size.height);
        self.depth_texture = Texture::create_depth(&self.device, &self.config, "depth_texture");
    }

    //
    //   Render Methods
    //

    pub fn render<V: VertexTrait, M: MeshTrait<V>>(&mut self, meshes: Vec<&M>, debug: bool) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        // merge meshes here
        // ...
        // make buffers here. They must outlive renderpass
        let mut buffers: Vec<(&wgpu::Buffer, &wgpu::Buffer, u32)> = Vec::new();
        for mesh in meshes{
            let vertex_buffer = mesh.get_vertices();
            let index_buffer = mesh.get_indices();
            buffers.push((vertex_buffer, index_buffer, mesh.get_num_indices()));
        }
        // need to change scope so it doesnt matter i
        // forget drop(render_pass)
        let shader = self.shaders.get(&M::get_shader()).unwrap();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None
            });

            render_pass.set_pipeline(shader.get_pipeline());
            let mut index = 0u32;
            for bind_group in shader.get_bind_groups(){
                render_pass.set_bind_group(index, bind_group, &[]);
                index += 1;
            }
            // remove this later since there will only be one vertex and index buffers
            for buffer_group in &buffers{
                let vertex_buffer_slice = buffer_group.0.slice(..);
                let index_buffer_slice = buffer_group.1.slice(..);

                render_pass.set_vertex_buffer(0, vertex_buffer_slice);
                render_pass.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint32); 
                render_pass.draw_indexed(0..buffer_group.2, 0, 0..1);
            } 
        }

        if debug{
            self.text_data.pre_render("Debug", &self.device, &self.queue)
                .expect("Error loading pre-render text data");
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Load data from Advanced Logging
            

            self.text_data.text_renderer.render(&self.text_data.atlas, &mut pass)
                .expect("Error rendering text data");
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    ///
    /// the stuff i made
    ///

    pub fn register_shader<V: VertexTrait>(&mut self, name: String, file_data: &str, bind_groups: Vec<wgpu::BindGroup>, bind_group_layouts: &[&wgpu::BindGroupLayout]){
        let shader = Shader::new::<V>(file_data, &self.device, &self.config, bind_groups, bind_group_layouts);
        self.shaders.insert(name, shader);
    }

    pub fn update_shader_bind_group(&mut self, name: String, index: usize, new_bind_group: wgpu::BindGroup){
        let shader = self.shaders.get_mut(&name).unwrap();
        shader.update_bind_group(index, new_bind_group);
    }
}