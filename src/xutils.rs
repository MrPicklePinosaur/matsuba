
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::*,
        render::*,
    },
    errors::ReplyOrIdError,
};
use fontconfig::Fontconfig;
use freetype::{Library, GlyphSlot, Face};
use freetype::face::LoadFlag;
use xmodmap::{KeySym, Modifier};

use super::error::{BoxResult, SimpleError};

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

fn get_font<C: Connection>(
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

pub fn create_face(font_name: &str) -> BoxResult<freetype::Face>{

    // load font using fontconfig
    let fc = Fontconfig::new().ok_or(Box::new(SimpleError::new("could not start fontconfig")))?;
    let font = fc.find(font_name, None).ok_or(Box::new(SimpleError::new("could not find font")))?;
    println!("{}: {}", font.name, font.path.display());

    // freetype init
    let lib = Library::init()?;
    let face = lib.new_face(font.path.as_os_str(), 0)?;
    face.set_char_size(40*64, 0, 50, 0)?;
    Ok(face)
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
    // println!("stride {} {} {}", glyphinfo.width, stride, glyphinfo.height);
    let input_bitmap = glyph.bitmap().buffer().to_owned();
    let mut output_bitmap = vec![0u8; (stride*glyphinfo.height) as usize];
    for y in 0..glyphinfo.height {
        output_bitmap[(y*stride) as usize..((y+1)*stride-(stride-glyphinfo.width)) as usize]
            .copy_from_slice(&input_bitmap[(y*glyphinfo.width) as usize..((y+1)*glyphinfo.width) as usize]);
    }
    // add_glyphs(conn, gsid, &[glyph_index], &[glyphinfo], &output_bitmap)?.check()?;
    add_glyphs(conn, gsid, &[glyph_index], &[glyphinfo], &input_bitmap)?.check()?;

    // debug_glyph(face.glyph());
    // println!("{:?}", glyphinfo);
    // println!("{:?}", output_bitmap);

    Ok(())
}

pub fn render_glyph<C: Connection>(
    conn: &C
) -> BoxResult<()> {


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

pub fn x_to_xmodmap_modifier(state: u16) -> Modifier {
    if state & u16::from(KeyButMask::SHIFT) == 0 {
        Modifier::Key
    } else {
        Modifier::ShiftKey
    }
}

pub fn xmodmap_to_x_modifier(modifier: Modifier) -> u16 {
    match modifier {
        Modifier::Key => 0,
        Modifier::ShiftKey => u16::from(KeyButMask::SHIFT),
        _ => 0, // TODO maybe should return error?
    }
}

