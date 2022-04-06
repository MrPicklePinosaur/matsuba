
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::render::*;
use x11rb::protocol::Event;
use fontconfig::Fontconfig;
use freetype::Library;
use freetype::face::LoadFlag;

mod converter;
mod conversion;
mod keycode;
mod keysym;
mod error;
mod x;

use converter::*;
use x::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let dfa = build_dfa();
    let mut c = Converter::new(&dfa);

    // for ch in "kopnnnichiha makkkkkunn desu AHAHAHHI".chars() {
    //     c.input_char(ch);
    // }

    let keymap = keycode::load_xmodmap().unwrap();

    // x11rb init
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    // create graphics context
    let foreground = conn.generate_id()?;
    let values_list = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .graphics_exposures(0);
    conn.create_gc(foreground, screen.root, &values_list)?;

    // create window
    let win = create_win(&conn, screen)?;

    conn.map_window(win)?;
    conn.flush()?;

    let points = [
        Point { x: 10, y: 10 },
        Point { x: 100, y: 100 },
    ];

    // init font stuff
    let fc = Fontconfig::new().unwrap();
    let font = fc.find("sans", None).unwrap();

    println!("{}: {}", font.name, font.path.display());

    let lib = Library::init()?;
    let face = lib.new_face(font.path.as_os_str(), 0)?;
    face.set_char_size(40*64, 0, 50, 0)?;

    face.load_char('あ' as usize, LoadFlag::RENDER)?;
    println!("{:?}", face.glyph().metrics());
    face.load_char('い' as usize, LoadFlag::RENDER)?;
    println!("{:?}", face.glyph().metrics());

    let pictformats = query_pict_formats(&conn)?.reply()?;
    println!("{:?}", pictformats);
    let pictformat = pictformats.formats[0];
    let glyphset = GlyphsetWrapper::create_glyph_set(&conn, pictformat.id)?;
    Ok(())

/*
    // main loop
    let mut running = true;
    while running {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(_event) => {
                conn.poly_line(CoordMode::PREVIOUS, win, foreground, &points)?; 
                conn.flush()?;
            }
            Event::KeyPress(event) => {
                let keysym = keymap.get(&(event.state,event.detail)).unwrap();
                // println!("keypress {}", keysym.as_char().unwrap());
                c.input_char(keysym.as_char().unwrap());
                draw_text(&conn, screen, win, 10, 140, "fixed", &c.output)?;
                conn.flush()?;
            }
            _ => {

            }
        }
    }

    drop(conn);
    Ok(())
*/
}

