
use x11rb::{
    COPY_DEPTH_FROM_PARENT, CURRENT_TIME,
    connection::Connection,
    protocol::{
        Event,
        xproto::*,
        render::*,
    },
    errors::ReplyOrIdError,
};
use fontconfig::Fontconfig;
use freetype::{Library, GlyphSlot, Face};
use freetype::face::LoadFlag;
use xmodmap::{KeyTable, Modifier, KeySym};

use super::error::{BoxResult, SimpleError};
use super::converter::{State, Converter};
use super::db;
use super::db::DBConnection;
use super::config;

pub struct XSession<'a, C: Connection> {
    conn: &'a C,
    screen: &'a Screen,
    completion_box: Option<Window>,
    completion_gc: Gcontext,
    keytable: KeyTable,
    converter: Converter<'a>,
    db_conn: DBConnection,
    running: bool,
}

impl<'a, C: Connection> XSession<'a, C> {

    pub fn new(conn: &'a C, screen: &'a Screen, dfa: &'a State) -> BoxResult<XSession<'a, C>> {

        let completion_gc = conn.generate_id()?;
        let values_list = CreateGCAux::new()
            .foreground(screen.white_pixel)
            .background(screen.black_pixel);
        conn.create_gc(completion_gc, screen.root, &values_list)?;

        let keytable = KeyTable::new()?;
        let converter = Converter::new(&dfa);
        let db_conn = db::get_connection()?;

        Ok(XSession {
            conn: conn,
            screen: screen,
            completion_box: None,
            completion_gc: completion_gc,
            keytable: keytable,
            converter: converter,
            db_conn: db_conn,
            running: true,
        })

    }

    pub fn configure_root(&self) -> BoxResult<()> {

        // append to root window attributes
        let attrs = self.conn.get_window_attributes(self.screen.root)?.reply()?;
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(attrs.your_event_mask|EventMask::SUBSTRUCTURE_NOTIFY); // TODO this might need to be attrs.all_event_masks
        self.conn.change_window_attributes(self.screen.root, &values_list)?.check()?;

        // grab user keypresses
        let grab_status = self.conn.grab_keyboard(false, self.screen.root, CURRENT_TIME, GrabMode::ASYNC, GrabMode::ASYNC)?.reply()?;
        if grab_status.status != GrabStatus::SUCCESS {
            return Err(Box::new(SimpleError::new("error grabbing keyboard")));
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event) -> BoxResult<()> {

        match event {
            Event::KeyPress(event) => {
                self.handle_keypress(event)?;
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) -> BoxResult<()> {

        let modifier = if event.state & u16::from(KeyButMask::SHIFT) == 0 { Modifier::Key } else { Modifier::ShiftKey };
        let keysym = self.keytable.get_keysym(modifier, event.detail);
        if keysym.is_err() { return Ok(()); }
        let keysym = keysym.unwrap();

        match keysym {
            KeySym::KEY_RETURN => {
                let output = self.converter.accept();

                /*
                // find currently focused window
                let focused_win = self.conn.get_input_focus()?.reply()?.focus;
                let keypress_event = KeyPressEvent {
                    // response_type:,
                    // detail:
                    
                }
                self.conn.send_event(false, focused_win, EventMask::KeyPress, )?;
                */

                self.running = false;
            }
            KeySym::KEY_BACKSPACE => {
                self.converter.del_char();
                println!("{}", self.converter.output);
            }
            KeySym::KEY_TAB => {
                let output = self.converter.accept();

                let converted = db::search(&self.db_conn, &output)?;

                if converted.len() == 0 {
                    println!("{}", output);
                } else {
                    println!("{}", converted.get(0).unwrap().k_ele);
                }
            }
            _ => {
                let ch = keysym.as_char();
                if ch.is_none() { return Ok(()); }
                self.converter.input_char(ch.unwrap());
                println!("{}", self.converter.output);
            }
        }

        Ok(())
    }

    pub fn create_completion_box(&mut self, position: (i16, i16), text: &str) -> BoxResult<()> {

        // TODO temp
        const TEXT_WIDTH: u16 = 5;

        // create completion box window
        let win = self.conn.generate_id()?;
        let values_list = CreateWindowAux::default()
            .background_pixel(self.screen.white_pixel)
            .override_redirect(1); // make window manager ignore the window
        self.conn.create_window(
            0, // draw on top of everything
            win,
            self.screen.root,
            position.0,
            position.1,
            text.len() as u16 * TEXT_WIDTH,
            config::COMPLETION_BOX_HEIGHT,
            0,
            WindowClass::INPUT_OUTPUT,
            self.screen.root_visual,
            &values_list,
        )?;

        self.completion_box = Some(win);

        Ok(())
    }

    // pub fn render_completion_box(&self, position: (i16, i16), text: &str) -> BoxResult<()> {
    pub fn render_completion_box(&self) -> BoxResult<()> {

        if self.completion_box.is_none() { return Ok(()); }
        self.conn.map_window(self.completion_box.unwrap())?;

        Ok(())
    }

    fn destroy_completion_box(&mut self) -> BoxResult<()> {
        if self.completion_box.is_none() { return Ok(()); }
        self.conn.destroy_window(self.completion_box.unwrap())?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

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
