
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;

mod converter;
mod conversion;
mod keycode;
mod keysym;
mod error;

use converter::*;
use keycode::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dfa = build_dfa();
    let mut c = Converter::new(&dfa);

    // for ch in "kopnnnichiha makkkkkunn desu AHAHAHHI".chars() {
    //     c.input_char(ch);
    // }

    let keymap = keycode::load_xmodmap().unwrap();

    // xcb init
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    // create graphics context
    let win = screen.root;
    let foreground = conn.generate_id()?;
    let values_list = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .graphics_exposures(0);
    conn.create_gc(foreground, win, &values_list)?;

    // create window
    let win = conn.generate_id()?;
    let values_list = CreateWindowAux::default()
        .background_pixel(screen.white_pixel)
        .event_mask(EventMask::EXPOSURE|EventMask::KEY_PRESS);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win,
        screen.root,
        0,
        0,
        150,
        150,
        10,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &values_list,
    )?;

    conn.map_window(win)?;
    conn.flush()?;

    let points = [
        Point { x: 10, y: 10 },
        Point { x: 100, y: 100 },
    ];

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(event) => {
                conn.poly_line(CoordMode::PREVIOUS, win, foreground, &points); 
                conn.flush()?;
            }
            Event::KeyPress(event) => {
                let keysym = keymap.get(&(event.state,event.detail)).unwrap();
                // println!("keypress {}", keysym.as_char().unwrap());
                c.input_char(keysym.as_char().unwrap());
                println!("{}", c.output);
            }
            _ => {

            }
        }
    }

    drop(conn);
    Ok(())
}

