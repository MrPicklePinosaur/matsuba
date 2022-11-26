use log::{debug, info};
use wgpu::include_wgsl;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, ModifiersState, *},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,

    pub output: String,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // format: surface.get_supported_formats(&adapter)[0],
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        // set up glyph brush
        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "fonts/SourceHanSansJP-Normal.otf"
        ))
        .expect("could not load font");
        let glyph_brush =
            wgpu_glyph::GlyphBrushBuilder::using_font(font).build(&device, render_format);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            staging_belt: wgpu::util::StagingBelt::new(1024),
            glyph_brush,

            output: String::new(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        drop(render_pass);

        self.glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (0., 0.),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![wgpu_glyph::Text::new(&self.output)
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(40.)],
            ..wgpu_glyph::Section::default()
        });

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                self.size.width,
                self.size.height,
            )
            .expect("Could not draw queued");

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();

        Ok(())
    }
}
