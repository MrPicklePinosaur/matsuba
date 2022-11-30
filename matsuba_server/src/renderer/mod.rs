mod gui;
mod util;

use log::{debug, info};
use wgpu::include_wgsl;
use wgpu_glyph::ab_glyph::{Font, FontArc, ScaleFont};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{ElementState, ModifiersState, *},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
};

use crate::renderer::gui::GUIState;
use crate::{
    config::{HENKAN_KEY, MUHENKAN_KEY},
    db,
};

use matsuba_common::converter::Converter;

use self::util::Key;

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
        .with_inner_size(LogicalSize::new(300.0, 30.0))
        .with_title("highgui")
        .with_decorations(false)
        .with_always_on_top(true)
        .with_resizable(false)
        .with_override_redirect(true)
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut gui_state = GUIState::new(&window).await;

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
            let mut clear_conversions = || {
                ime_state.conversions.clear();
                gui_state.conversions.clear();
                ime_state.selected_conversion = 0;
                update_size(&gui_state, &window);
            };

            match event {
                DeviceEvent::Key(KeyboardInput {
                    state,
                    virtual_keycode,
                    modifiers,
                    ..
                }) if state == ElementState::Pressed => {
                    if let Some(virtual_keycode) = virtual_keycode {
                        if !ime_state.henkan {
                            match virtual_keycode {
                                HENKAN_KEY => {
                                    info!("henkan");
                                    ime_state.henkan = true;
                                }
                                _ => {}
                            }
                        } else {
                            match virtual_keycode {
                                MUHENKAN_KEY => {
                                    info!("muhenkan");
                                    ime_state.henkan = false;

                                    // TOOD duplicate of return rn
                                    converter.accept();
                                    clear_conversions();

                                    gui_state.output = String::new();
                                    window.set_visible(false);
                                }
                                VirtualKeyCode::Return => {
                                    info!("accepting: {}", converter.output);

                                    converter.accept();
                                    clear_conversions();

                                    gui_state.output = String::new();
                                    window.set_visible(false);
                                }
                                VirtualKeyCode::Back => {
                                    converter.del_char();

                                    // we changed input so clear conversions
                                    clear_conversions();

                                    gui_state.output = converter.output.clone();
                                    info!("deleted {:?}", converter.output);
                                }
                                VirtualKeyCode::Escape => {
                                    // cancel out of conversion
                                    clear_conversions();

                                    // bring back raw kana
                                    gui_state.output = converter.output.clone();
                                }
                                VirtualKeyCode::Tab => {
                                    // conversion already done, cycle through options
                                    if ime_state.conversions.len() > 0 {
                                        if !modifiers.shift() {
                                            ime_state.selected_conversion =
                                                (ime_state.selected_conversion + 1)
                                                    % (ime_state.conversions.len());
                                        } else {
                                            ime_state.selected_conversion = (ime_state
                                                .selected_conversion
                                                + ime_state.conversions.len()
                                                - 1)
                                                % (ime_state.conversions.len());
                                        };
                                        info!("new index {}", ime_state.selected_conversion);
                                    } else {
                                        // conversion not done, populate conversion options list
                                        let db_conn = db::get_connection().unwrap();
                                        let kana = &converter.output;
                                        let converted = db::search(&db_conn, kana).unwrap();

                                        for entry in converted {
                                            ime_state.conversions.push(entry.k_ele);
                                        }

                                        // always push exactly what we typed
                                        // TODO having duplicate ime_state.conversions and gui_state.conversions is very bad
                                        ime_state.conversions.push(kana.clone());
                                        gui_state.conversions = ime_state.conversions.clone();

                                        // set current to beginning
                                        ime_state.selected_conversion = 0;
                                        info!("conversions {:?}", ime_state.conversions);
                                    }
                                    gui_state.output = ime_state
                                        .conversions
                                        .get(ime_state.selected_conversion)
                                        .unwrap()
                                        .to_string();
                                    update_size(&gui_state, &window);
                                }
                                _ => {
                                    // otherwise feed input directly to converter
                                    if let Ok(c) = char::try_from(Key(virtual_keycode, modifiers)) {
                                        converter.input_char(c);

                                        // we changed input so clear conversions
                                        clear_conversions();

                                        gui_state.output = converter.output.clone();
                                        info!("inputted {:?}", converter.output);

                                        // show completion box
                                        window.set_visible(true);
                                        update_size(&gui_state, &window);
                                    }
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

fn update_size(gui_state: &GUIState, window: &Window) {
    let scaled_font = gui_state.font.as_scaled(gui_state.font_scale);

    // let min_font_size = scaled_font.h_advance(gui_state.font.glyph_id('あ')); // value of 27.62431

    // calculate max horizontal
    let total_width = 300.;

    // calculate max vertical
    let total_height = scaled_font.height() * (gui_state.conversions.len() as f32 + 1.0);

    window.set_inner_size(PhysicalSize {
        width: total_width,
        height: total_height,
    });
}
