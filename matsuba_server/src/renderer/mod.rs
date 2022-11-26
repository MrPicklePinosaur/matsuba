mod gui;

use log::{debug, info};
use wgpu::include_wgsl;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, ModifiersState, *},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
};

use gui::State;

use matsuba_common::converter::Converter;

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

    let mut gui_state = State::new(&window).await;

    let mut ime_state = IMEState::new();
    let mut converter = Converter::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } => {
            if window_id == window.id() && !gui_state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        gui_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        gui_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            gui_state.update();
            match gui_state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => gui_state.resize(gui_state.size),
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
                                gui_state.output = converter.output.clone();
                                info!("deleted {:?}", converter.output);
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
                                    gui_state.output = converter.output.clone();
                                    info!("inputted {:?}", converter.output);
                                }
                            }
                        }
                    };
                }
                _ => {}
            }
        }
        _ => {}
    });
}

// TODO make this nicer
fn virtual_to_char(k: VirtualKeyCode, m: ModifiersState) -> Option<char> {
    let byte = match k {
        VirtualKeyCode::A if !m.shift() => 0x61,
        VirtualKeyCode::B if !m.shift() => 0x62,
        VirtualKeyCode::C if !m.shift() => 0x63,
        VirtualKeyCode::D if !m.shift() => 0x64,
        VirtualKeyCode::E if !m.shift() => 0x65,
        VirtualKeyCode::F if !m.shift() => 0x66,
        VirtualKeyCode::G if !m.shift() => 0x67,
        VirtualKeyCode::H if !m.shift() => 0x68,
        VirtualKeyCode::I if !m.shift() => 0x69,
        VirtualKeyCode::J if !m.shift() => 0x6a,
        VirtualKeyCode::K if !m.shift() => 0x6b,
        VirtualKeyCode::L if !m.shift() => 0x6c,
        VirtualKeyCode::M if !m.shift() => 0x6d,
        VirtualKeyCode::N if !m.shift() => 0x6e,
        VirtualKeyCode::O if !m.shift() => 0x6f,
        VirtualKeyCode::P if !m.shift() => 0x70,
        VirtualKeyCode::Q if !m.shift() => 0x71,
        VirtualKeyCode::R if !m.shift() => 0x72,
        VirtualKeyCode::S if !m.shift() => 0x73,
        VirtualKeyCode::T if !m.shift() => 0x74,
        VirtualKeyCode::U if !m.shift() => 0x75,
        VirtualKeyCode::V if !m.shift() => 0x76,
        VirtualKeyCode::W if !m.shift() => 0x77,
        VirtualKeyCode::X if !m.shift() => 0x78,
        VirtualKeyCode::Y if !m.shift() => 0x79,
        VirtualKeyCode::Z if !m.shift() => 0x7a,
        VirtualKeyCode::A if m.shift() => 0x41,
        VirtualKeyCode::B if m.shift() => 0x42,
        VirtualKeyCode::C if m.shift() => 0x43,
        VirtualKeyCode::D if m.shift() => 0x44,
        VirtualKeyCode::E if m.shift() => 0x45,
        VirtualKeyCode::F if m.shift() => 0x46,
        VirtualKeyCode::G if m.shift() => 0x47,
        VirtualKeyCode::H if m.shift() => 0x48,
        VirtualKeyCode::I if m.shift() => 0x49,
        VirtualKeyCode::J if m.shift() => 0x4a,
        VirtualKeyCode::K if m.shift() => 0x4b,
        VirtualKeyCode::L if m.shift() => 0x4c,
        VirtualKeyCode::M if m.shift() => 0x4d,
        VirtualKeyCode::N if m.shift() => 0x4e,
        VirtualKeyCode::O if m.shift() => 0x4f,
        VirtualKeyCode::P if m.shift() => 0x50,
        VirtualKeyCode::Q if m.shift() => 0x51,
        VirtualKeyCode::R if m.shift() => 0x52,
        VirtualKeyCode::S if m.shift() => 0x53,
        VirtualKeyCode::T if m.shift() => 0x54,
        VirtualKeyCode::U if m.shift() => 0x55,
        VirtualKeyCode::V if m.shift() => 0x56,
        VirtualKeyCode::W if m.shift() => 0x57,
        VirtualKeyCode::X if m.shift() => 0x58,
        VirtualKeyCode::Y if m.shift() => 0x59,
        VirtualKeyCode::Z if m.shift() => 0x5a,
        _ => 0x00,
    };
    char::from_u32(byte)
}
