mod gui;
mod util;

#[cfg(feature = "x11")]
mod xorg;
use pino_utils::ok_or_return;
use pino_xmodmap::{KeySym, Modifier};

use log::{error, info};

use wgpu_glyph::ab_glyph::{Font, ScaleFont};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
};
use x11rb::protocol::xproto::KeyButMask;

use crate::{
    config::{HENKAN_KEY, MUHENKAN_KEY},
    db,
};
use crate::{output, renderer::gui::GUIState};

use matsuba_common::converter::Converter;

pub(crate) struct IMEState {
    pub selected_conversion: usize,
    pub conversions: Vec<String>,
    pub output: String,
    pub henkan: bool,
}

impl IMEState {
    pub fn new() -> Self {
        IMEState {
            selected_conversion: 0,
            conversions: vec![],
            output: String::new(),
            henkan: false,
        }
    }

    pub fn clear_conversions(&mut self) {
        self.conversions.clear();
        self.selected_conversion = 0;
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

    let xsession = xorg::XSession::new().expect("failed getting xsession");
    xsession.configure_root().expect("could not configure root");
    xsession.ungrab_keyboard().unwrap(); // only since we start in muhenkan mode

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
            match gui_state.render(&ime_state) {
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
        Event::DeviceEvent {
            device_id: _,
            event: _,
        } => {}
        _ => {
            // now run our own keyboard code
            let (modifier, keysym) = ok_or_return!(xsession.handle_keypress());

            if !ime_state.henkan {
                match keysym {
                    HENKAN_KEY => {
                        info!("henkan");
                        ime_state.henkan = true;
                        xsession.grab_keyboard().expect("could not grab kb");
                    }
                    _ => {}
                };
            } else {
                match keysym {
                    MUHENKAN_KEY => {
                        info!("muhenkan");
                        ime_state.henkan = false;

                        xsession.ungrab_keyboard().expect("could not ungrab kb");

                        // TOOD duplicate of return rn
                        converter.accept();
                        ime_state.clear_conversions();
                        ime_state.conversions.clear();
                        ime_state.selected_conversion = 0;
                        update_size(&gui_state, &ime_state, &window);

                        ime_state.output = String::new();
                        window.set_visible(false);
                    }
                    KeySym::KEY_RETURN => {
                        info!("accepting: {}", converter.output);

                        xsession.ungrab_keyboard().unwrap();

                        let output = if let Some(output) =
                            ime_state.conversions.get(ime_state.selected_conversion)
                        {
                            output
                        } else {
                            &converter.output
                        };

                        if let Err(e) = output::output(output) {
                            error!("{:?}", e);
                        }

                        xsession.grab_keyboard().unwrap();

                        converter.accept();
                        ime_state.clear_conversions();
                        update_size(&gui_state, &ime_state, &window);

                        ime_state.output = String::new();
                        window.set_visible(false);
                    }
                    KeySym::KEY_BACKSPACE => {
                        converter.del_char();

                        // we changed input so clear conversions
                        ime_state.clear_conversions();
                        update_size(&gui_state, &ime_state, &window);

                        ime_state.output = converter.output.clone();
                        info!("deleted {:?}", converter.output);

                        // if input empty now close window
                        if ime_state.output.is_empty() {
                            window.set_visible(false);
                        }
                    }
                    KeySym::KEY_ESCAPE => {
                        if ime_state.conversions.is_empty() {
                            // if conversion already empty, close conversion window and reset entire conversion

                            converter.accept();
                            ime_state.clear_conversions();
                            update_size(&gui_state, &ime_state, &window);

                            ime_state.output = String::new();
                            window.set_visible(false);
                        } else {
                            // otherwise cancel out of conversion

                            ime_state.clear_conversions();
                            update_size(&gui_state, &ime_state, &window);

                            // bring back raw kana
                            ime_state.output = converter.output.clone();
                        }
                    }
                    KeySym::KEY_TAB => {
                        // conversion already done, cycle through options
                        if !ime_state.conversions.is_empty() {
                            if modifier == KeyButMask::SHIFT {
                                ime_state.selected_conversion = (ime_state.selected_conversion + 1)
                                    % (ime_state.conversions.len());
                            } else {
                                ime_state.selected_conversion = (ime_state.selected_conversion
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
                            ime_state.conversions.push(kana.clone());

                            // set current to beginning
                            ime_state.selected_conversion = 0;
                            info!("conversions {:?}", ime_state.conversions);
                        }
                        ime_state.output = ime_state
                            .conversions
                            .get(ime_state.selected_conversion)
                            .unwrap()
                            .to_string();
                        update_size(&gui_state, &ime_state, &window);
                    }
                    _ => {
                        // otherwise feed input directly to converter
                        if let Some(c) = keysym.as_char() {
                            // TODO fix pino_xmodmap library to not return null characters
                            if c != '\0' {
                                converter.input_char(c);

                                // we changed input so clear conversions
                                ime_state.clear_conversions();

                                ime_state.output = converter.output.clone();
                                info!("inputted {:?}", converter.output);

                                // show completion box
                                window.set_visible(true);
                                update_size(&gui_state, &ime_state, &window);
                            }
                        }
                    }
                };
            };
        }
    });
}

fn update_size(gui_state: &GUIState, ime_state: &IMEState, window: &Window) {
    let scaled_font = gui_state.font.as_scaled(gui_state.font_scale);

    // let min_font_size = scaled_font.h_advance(gui_state.font.glyph_id('あ')); // value of 27.62431

    // calculate max horizontal
    let total_width = 300.;

    // calculate max vertical
    let total_height = scaled_font.height() * (ime_state.conversions.len() as f32 + 1.0);

    window.set_inner_size(PhysicalSize {
        width: total_width,
        height: total_height,
    });
}
