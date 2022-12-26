use wgpu_glyph::ab_glyph::{Font, FontArc, ScaleFont};
use winit::{event::*, window::Window};

use crate::config::SETTINGS;

use super::IMEState;

pub(crate) struct GUIState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    staging_belt: wgpu::util::StagingBelt,
    pub font: FontArc,
    pub font_scale: f32,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
    shape_renderer: pino_wgpu_shape::ShapeRenderer,
}

impl GUIState {
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
            wgpu_glyph::GlyphBrushBuilder::using_font(font.clone()).build(&device, render_format);

        let shape_renderer = pino_wgpu_shape::ShapeRenderer::new(&device, render_format);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            staging_belt: wgpu::util::StagingBelt::new(1024),
            font,
            font_scale: 40.0,
            glyph_brush,
            shape_renderer,
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

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, ime_state: &IMEState) -> Result<(), wgpu::SurfaceError> {
        // TODO constants move to config later
        let bg_color = SETTINGS.theme.bg.as_slice_rgba();
        let cur_color = Vector3::from(SETTINGS.theme.selected_bg.as_slice_rgb());
        let hl_color = Vector3::from(SETTINGS.theme.completion_bg.as_slice_rgb());

        let output = self.surface.get_current_texture()?;
        let mut view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: bg_color[0] as f64,
                        g: bg_color[1] as f64,
                        b: bg_color[2] as f64,
                        a: bg_color[3] as f64,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        drop(render_pass);

        // draw box
        let columns = 1.0 + ime_state.conversions.len() as f32;
        use cgmath::Vector3;
        use pino_wgpu_shape::Instance;

        let instances = vec![
            // selected conversion hightlight
            Instance {
                position: Vector3::new(0., 1. - 1. / columns, 0.),
                scale: Vector3::new(1., 1. / columns, 1.),
                color: cur_color,
            },
            // conversion highlight
            Instance {
                position: Vector3::new(
                    0.,
                    (1. - 1. / columns)
                        - (2. / columns * (ime_state.selected_conversion as f32 + 1.)),
                    0.,
                ),
                scale: Vector3::new(1., 1. / columns, 1.),
                color: hl_color,
            },
        ];
        for instance in instances {
            self.shape_renderer.queue(instance);
        }
        self.shape_renderer.draw(
            &self.device,
            &mut encoder,
            &mut view,
            &mut self.staging_belt,
        );

        // draw selected text
        self.glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (0., 0.),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![wgpu_glyph::Text::new(&ime_state.output)
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(self.font_scale)],
            ..wgpu_glyph::Section::default()
        });

        // draw all completions
        let scaled_font = self.font.as_scaled(self.font_scale);
        for (i, conversion) in ime_state.conversions.iter().enumerate() {
            self.glyph_brush.queue(wgpu_glyph::Section {
                screen_position: (0., scaled_font.height() * ((i as f32) + 1.)),
                bounds: (self.size.width as f32, self.size.height as f32),
                text: vec![wgpu_glyph::Text::new(conversion)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(self.font_scale)],
                ..wgpu_glyph::Section::default()
            });
        }

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
