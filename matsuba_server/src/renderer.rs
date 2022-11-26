use log::{debug, info};
use wgpu::include_wgsl;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, ModifiersState, *},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
};

use matsuba_common::converter::Converter;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
}

impl State {
    async fn new(window: &Window) -> Self {
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
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            screen_position: (30., 30.),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![wgpu_glyph::Text::new("Hello world ひらがな")
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

struct IMEState {
    selected_conversion: usize,
    conversions: Vec<String>,
    henkan: bool,
}

impl IMEState {
    pub fn new() -> Self {
        IMEState {
            selected_conversion: 0,
            conversions: vec![],
            henkan: false,
        }
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        // .with_position(position)
        // .with_inner_size(size)
        .with_title("highgui")
        .with_decorations(false)
        .with_always_on_top(true)
        .with_resizable(false)
        .with_override_redirect(true)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window).await;

    let mut ime_state = IMEState::new();
    let mut converter = Converter::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => {
            if window_id == window.id() && !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        Event::DeviceEvent { device_id, event } => {
            // events that are recieved regardless of focus
            match event {
                DeviceEvent::Key(KeyboardInput {
                    state,
                    virtual_keycode,
                    modifiers,
                    ..
                }) if state == ElementState::Pressed => {
                    if let Some(virtual_keycode) = virtual_keycode {
                        match virtual_keycode {
                            VirtualKeyCode::Return => {
                                info!("accepting: {}", converter.output);

                                converter.accept();
                                ime_state.conversions.clear();
                                ime_state.selected_conversion = 0;
                            }
                            VirtualKeyCode::Back => {
                                converter.del_char();
                            }
                            VirtualKeyCode::Escape => {
                                // cancel out of conversion
                                ime_state.conversions.clear();
                                ime_state.selected_conversion = 0;
                            }
                            VirtualKeyCode::Tab if !modifiers.shift() => {
                                // cycle conversions
                                if ime_state.conversions.len() > 0 {
                                    ime_state.selected_conversion = (ime_state.selected_conversion
                                        + 1)
                                        % (ime_state.conversions.len());
                                }
                            }
                            VirtualKeyCode::Tab if modifiers.shift() => {
                                if ime_state.conversions.len() > 0 {
                                    ime_state.selected_conversion = (ime_state.selected_conversion
                                        - 1)
                                        % (ime_state.conversions.len());
                                }
                            }
                            _ => {
                                // otherwise feed input directly to converter
                                if let Some(c) = virtual_to_char(virtual_keycode, modifiers) {
                                    converter.input_char(c);
                                    info!("inputted {:?}", converter.output);
                                }
                            }
                        }
                    };
                    // println!("DeviceInput Key {:?}", kb_input);
                }
                _ => {}
            }
        }
        _ => {}
    });
}

fn virtual_to_char(k: VirtualKeyCode, m: ModifiersState) -> Option<char> {
    let byte = if !m.shift() {
        match k {
            VirtualKeyCode::A => 0x61,
            VirtualKeyCode::B => 0x62,
            VirtualKeyCode::C => 0x63,
            VirtualKeyCode::D => 0x64,
            VirtualKeyCode::E => 0x65,
            VirtualKeyCode::F => 0x66,
            VirtualKeyCode::G => 0x67,
            VirtualKeyCode::H => 0x68,
            VirtualKeyCode::I => 0x69,
            VirtualKeyCode::J => 0x6a,
            VirtualKeyCode::K => 0x6b,
            VirtualKeyCode::L => 0x6c,
            VirtualKeyCode::M => 0x6d,
            VirtualKeyCode::N => 0x6e,
            VirtualKeyCode::O => 0x6f,
            VirtualKeyCode::P => 0x70,
            VirtualKeyCode::Q => 0x71,
            VirtualKeyCode::R => 0x72,
            VirtualKeyCode::S => 0x73,
            VirtualKeyCode::T => 0x74,
            VirtualKeyCode::U => 0x75,
            VirtualKeyCode::V => 0x76,
            VirtualKeyCode::W => 0x77,
            VirtualKeyCode::X => 0x78,
            VirtualKeyCode::Y => 0x79,
            VirtualKeyCode::Z => 0x7a,
            _ => 0x00,
        }
    } else if m.shift() {
        match k {
            VirtualKeyCode::A => 0x41,
            VirtualKeyCode::B => 0x42,
            VirtualKeyCode::C => 0x43,
            VirtualKeyCode::D => 0x44,
            VirtualKeyCode::E => 0x45,
            VirtualKeyCode::F => 0x46,
            VirtualKeyCode::G => 0x47,
            VirtualKeyCode::H => 0x48,
            VirtualKeyCode::I => 0x49,
            VirtualKeyCode::J => 0x4a,
            VirtualKeyCode::K => 0x4b,
            VirtualKeyCode::L => 0x4c,
            VirtualKeyCode::M => 0x4d,
            VirtualKeyCode::N => 0x4e,
            VirtualKeyCode::O => 0x4f,
            VirtualKeyCode::P => 0x50,
            VirtualKeyCode::Q => 0x51,
            VirtualKeyCode::R => 0x52,
            VirtualKeyCode::S => 0x53,
            VirtualKeyCode::T => 0x54,
            VirtualKeyCode::U => 0x55,
            VirtualKeyCode::V => 0x56,
            VirtualKeyCode::W => 0x57,
            VirtualKeyCode::X => 0x58,
            VirtualKeyCode::Y => 0x59,
            VirtualKeyCode::Z => 0x5a,
            _ => 0x00,
        }
    } else {
        0x00
    };
    char::from_u32(byte)
}
