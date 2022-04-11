
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
mod db;
mod xmlparse;
mod cli;
mod x;
mod argparse;
mod config;

use converter::*;
use x::*;
use db::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = std::path::Path::new("./tests/jmdict_full.xml");

    let mut conn = db::get_connection()?;
    // db::init(&conn)?;
    // xmlparse::parse_jmdict_xml(&mut conn, path)?;

    Ok(())
    /*
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

    // query pictformats
    let pictformats = query_pict_formats(&conn)?.reply()?;
    let format = pictformats.formats[0];

    // create picture
    let pid = conn.generate_id()?;
    let values_list = CreatePictureAux::default()
        .polymode(PolyMode::IMPRECISE)
        .polyedge(PolyEdge::SMOOTH);
    create_picture(&conn, pid, win, format.id, &values_list)?;

    // init font stuff
    let fc = Fontconfig::new().unwrap();
    let font = fc.find("sans", None).unwrap();

    println!("{}: {}", font.name, font.path.display());

    let lib = Library::init()?;
    let face = lib.new_face(font.path.as_os_str(), 0)?;
    face.set_char_size(40*64, 0, 50, 0)?;

    face.load_char('あ' as usize, LoadFlag::RENDER)?;
    let glyph_metrics = face.glyph().metrics();
    println!("{:?}", glyph_metrics);

    // convert freetype glyph to xcb glyph
    let gsid = conn.generate_id()?;
    create_glyph_set(&conn, gsid, format.id)?;

    // see https://freetype.org/freetype2/docs/glyphs/glyphs-3.html#section-3 for info on what each field is
    let glyphinfo = Glyphinfo {
        width: glyph_metrics.width as u16,
        height: glyph_metrics.height as u16,
        x: glyph_metrics.horiBearingX as i16,
        y: glyph_metrics.horiBearingY as i16,
        x_off: glyph_metrics.horiAdvance as i16,
        y_off: glyph_metrics.vertAdvance as i16,
    };
    add_glyphs(&conn, gsid, &['あ' as u32], &[glyphinfo], face.glyph().bitmap().buffer())?;
    composite_glyphs8(&conn, PictOp::OVER, foreground, pid, format.id, gsid, 100, 100, &['あ' as u8])?;

    // TODO free stuff

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

