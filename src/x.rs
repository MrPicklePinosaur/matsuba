
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::errors::ReplyOrIdError;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::protocol::render::*;
use x11rb::protocol::Event;
use x11rb::CURRENT_TIME;
use fontconfig::Fontconfig;
use freetype::{Library, GlyphSlot, Face};
use freetype::face::LoadFlag;

use xmodmap::{KeyTable, Modifier};
use super::error::BoxResult;

pub fn run_x() -> BoxResult<()> {

    let keytable = KeyTable::new()?;

    // x11rb init
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let values_list = ChangeWindowAttributesAux::default()
        .event_mask(EventMask::EXPOSURE|EventMask::BUTTON_PRESS);
    conn.change_window_attributes(screen.root, &values_list)?;

    grab_keyboard(&conn, true, screen.root, CURRENT_TIME, GrabMode::ASYNC, GrabMode::ASYNC)?;

    /*
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

    // query pictformats
    let pictformats = query_pict_formats(&conn)?.reply()?;
    // TODO hardcoded pictformat for now
    let format = pictformats.formats.iter().find(|f| f.id == 41).unwrap();
    // for pf in pictformats.formats {
    //     println!("{:?}", pf);
    // }

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

    // freetype init
    let lib = Library::init()?;
    let face = lib.new_face(font.path.as_os_str(), 0)?;
    face.set_char_size(40*64, 0, 50, 0)?;

    // xcb glyph init
    let gsid = conn.generate_id()?;
    create_glyph_set(&conn, gsid, format.id)?;
    create_glyph(&conn, &face, gsid, 'あ')?;
    */

    // TODO free stuff

    // main loop
    let mut running = true;
    while running {
        let event = conn.wait_for_event()?;
        match event {
            Event::Expose(_event) => {
                // let glyph_index = face.get_char_index('あ' as usize);
                // composite_glyphs32(&conn, PictOp::OVER, foreground, pid, format.id, gsid, 100, 100, &[glyph_index as u8])?;
                conn.flush()?;
            }
            Event::KeyPress(event) => {
                let modifier = if event.state & u16::from(KeyButMask::SHIFT) == 0 { Modifier::Key } else { Modifier::ShiftKey };
                let keysym = keytable.get_keysym(modifier,event.detail);
                if keysym.is_err() { break; }
                let keysym = keysym.unwrap();

                println!("keypress {}", keysym.as_char().unwrap());
                // c.input_char(keysym.as_char().unwrap());
                // draw_text(&conn, screen, win, 10, 140, "fixed", &"pee")?;
                conn.flush()?;
            }
            _ => {

            }
        }
    }

    drop(&conn);
    Ok(())
}

pub fn create_win<C: Connection>(
    conn: &C,
    screen: &Screen,
) -> Result<Window, ReplyOrIdError> {

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

    Ok(win)
}

pub fn create_glyph<C: Connection>(
    conn: &C,
    face: &Face,
    gsid: u32, // glyph set
    character: char
) -> BoxResult<()> {

    let glyph_index = face.get_char_index(character as usize);
    face.load_glyph(glyph_index, LoadFlag::RENDER)?;

    let glyph = face.glyph();

    // see https://freetype.org/freetype2/docs/glyphs/glyphs-3.html#section-3 for info on what each field is
    let glyphinfo = Glyphinfo {
        x: -glyph.bitmap_left() as i16,
        y: glyph.bitmap_top() as i16,
        width: glyph.bitmap().width() as u16,
        height: glyph.bitmap().rows() as u16,
        x_off: (glyph.advance().x/64) as i16,
        y_off: (glyph.advance().y/64) as i16,
    };

    // copy freetype bitmap to xcb (this code is very sketchy lmao)
    let stride = (glyphinfo.width+3)&!3;
    // println!("stride {}", stride);
    let input_bitmap = glyph.bitmap().buffer().to_owned();
    let mut output_bitmap = vec![0u8; (stride*glyphinfo.height) as usize];
    for y in 0..glyphinfo.height {
        output_bitmap[(y*stride) as usize..((y+1)*stride-(stride-glyphinfo.width)) as usize]
            .copy_from_slice(&input_bitmap[(y*glyphinfo.width) as usize..((y+1)*glyphinfo.width) as usize]);
    }
    add_glyphs(conn, gsid, &[glyph_index], &[glyphinfo], &output_bitmap)?;

    // debug_glyph(face.glyph());
    println!("{:?}", glyphinfo);
    println!("{:?}", output_bitmap);

    Ok(())
}

pub fn render_glyph<C: Connection>(
    conn: &C
) -> BoxResult<()> {


    Ok(())
}

pub fn get_font<C: Connection>(
    conn: &C,
    screen: &Screen,
    win: Window,
    font_name: &str
) -> Result<Gcontext, ReplyOrIdError> {

    let font = conn.generate_id()?;
    conn.open_font(font, font_name.as_bytes())?;

    let gc = conn.generate_id()?;
    let values_list = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .background(screen.white_pixel)
        .font(font);
    conn.create_gc(gc, win, &values_list)?;
    conn.close_font(font)?;
    
    Ok(gc)
}

pub fn draw_text<C:Connection>(
    conn: &C,
    screen: &Screen,
    win: Window,
    x: i16,
    y: i16,
    font_name: &str,
    text: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let gc = get_font(conn, screen, win, font_name)?;
    conn.image_text8(win, gc, x, y, text.as_bytes())?;
    conn.free_gc(gc)?;
    Ok(())
}

// glyph debug functions from https://github.com/PistonDevelopers/freetype-rs/blob/master/examples/single_glyph.rs
const WIDTH: usize = 32;
const HEIGHT: usize = 24;
fn draw_bitmap(bitmap: freetype::Bitmap, x: usize, y: usize) -> [[u8; WIDTH]; HEIGHT] {

    let mut figure = [[0; WIDTH]; HEIGHT];
    let mut p = 0;
    let mut q = 0;
    let w = bitmap.width() as usize;
    let x_max = x + w;
    let y_max = y + bitmap.rows() as usize;

    for i in x .. x_max {
        for j in y .. y_max {
            if i < WIDTH && j < HEIGHT {
                figure[j][i] |= bitmap.buffer()[q * w + p];
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }
    figure
}

fn debug_glyph(glyph: &GlyphSlot) {

    let x = glyph.bitmap_left() as usize;
    let y = HEIGHT - glyph.bitmap_top() as usize;
    let figure = draw_bitmap(glyph.bitmap(), x, y);

    for i in 0 .. HEIGHT {
        for j in 0 .. WIDTH {
            print!("{}",
                match figure[i][j] {
                    p if p == 0 => " ",
                    p if p < 128 => "*",
                    _  => "+"
                }
            );
        }
        println!("");
    }

}
