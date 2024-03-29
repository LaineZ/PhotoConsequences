use egui::TextureId;
use egui_wgpu_backend::RenderPass;
use image::RgbaImage;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, Texture};
use winit::window::Window;

pub enum Event {
    RequestRedraw,
}

/// This is the repaint signal type that egui needs for requesting a repaint from another thread.
/// It sends the custom RequestRedraw event to the winit event loop.
struct RepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for RepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

pub struct Renderer {
    /// Opened editor windows
    pub windows: Vec<Window>,
    /// Image preview texture
    pub texture: Option<egui::TextureId>,
    pub texture_native: Option<Texture>,
    pub device: Device,
    pub queue: Queue,
    pub render_pass: RenderPass,
    pub surface: Surface,
    pub surface_config: SurfaceConfiguration,
}

impl Renderer {
    pub fn new(window: &Window) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| anyhow::anyhow!("No supported wgpu adapters present in system."))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))?;

        let size = window.inner_size();
        let surface_format = surface.get_supported_formats(&adapter)[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let render_pass = RenderPass::new(&device, surface_format, 1);

        Ok(Self {
            windows: Vec::new(),
            texture: None,
            texture_native: None,
            device,
            queue,
            surface,
            surface_config,
            render_pass,
        })
    }

    pub fn close_render(&mut self) {
        self.windows.clear();
        std::process::exit(0)
    }

    pub fn cleanup_image(&mut self) {
        self.texture = None;
    }

    pub fn destroy_texture(&mut self) {
        if self.texture_native.is_some() {
            self.texture_native.as_ref().unwrap().destroy();
            self.texture_native = None;
        }
    }

    pub fn upload_texture(&mut self, image: &RgbaImage) -> TextureId {
        let texture_size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        self.texture_native = Some(self.device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
        }));

        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: self.texture_native.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            image,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * image.width()),
                rows_per_image: std::num::NonZeroU32::new(image.height()),
            },
            texture_size,
        );

        let view = self
            .texture_native
            .as_ref()
            .unwrap()
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.render_pass.egui_texture_from_wgpu_texture(
            &self.device,
            &view,
            wgpu::FilterMode::Nearest,
        )
    }
}
