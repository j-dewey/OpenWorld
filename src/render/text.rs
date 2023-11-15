use glyphon;

pub const TEXT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

// stuff needed to render text
pub struct TextData{
    font_system: glyphon::FontSystem,
    cache: glyphon::SwashCache,
    atlas: glyphon::TextAtlas,
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

        Self { font_system, cache, atlas, buffer, resolution }
    }

    pub fn resize(&mut self, scrn_width: u32, scrn_height: u32){
        self.buffer.set_size(&mut self.font_system, scrn_width as f32, scrn_height as f32);
        self.resolution = glyphon::Resolution{
            width: scrn_width,
            height: scrn_height
        };
    }
}