use glyphon;

pub const TEXT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

// stuff needed to render text
pub struct TextData{
    font_system: glyphon::FontSystem,
    cache: glyphon::SwashCache,
    pub(crate) atlas: glyphon::TextAtlas,
    pub(crate) text_renderer: glyphon::TextRenderer,
    buffer: glyphon::Buffer,
    resolution: glyphon::Resolution
}

impl TextData{
    pub fn new(scrn_width: u32, scrn_height: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Self{
        let mut font_system = glyphon::FontSystem::new();
        let mut cache = glyphon::SwashCache::new();
        let mut atlas = glyphon::TextAtlas::new(device, queue, TEXT_FORMAT);
        let mut text_renderer =
            glyphon::TextRenderer::new(&mut atlas, &device, wgpu::MultisampleState::default(), None);
        let mut buffer = glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(30.0, 42.0));
        
        buffer.set_size(&mut font_system, scrn_width as f32, scrn_height as f32);
        buffer.shape_until_scroll(&mut font_system);

        let resolution = glyphon::Resolution{
            width: scrn_width,
            height: scrn_height
        };

        Self { font_system, cache, atlas, text_renderer, buffer, resolution }
    }

    pub fn resize(&mut self, scrn_width: u32, scrn_height: u32){
        self.buffer.set_size(&mut self.font_system, scrn_width as f32, scrn_height as f32);
        self.resolution = glyphon::Resolution{
            width: scrn_width,
            height: scrn_height
        };
    }

    pub fn pre_render(&mut self, text: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), glyphon::PrepareError>{
        self.buffer.set_text(&mut self.font_system, text,  glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
        self.text_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            self.resolution,
            [glyphon::TextArea {
                buffer: &self.buffer,
                left: 10.0,
                top: 10.0,
                scale: 1.0,
                    bounds: glyphon::TextBounds {
                        left: 0,
                        top: 0,
                        right: 600,
                        bottom: 160,
                    },
               default_color: glyphon::Color::rgb(255, 255, 255),
            }],
            &mut self.cache,
        )
    }
}