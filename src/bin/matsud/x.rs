
use x11rb::{
    CURRENT_TIME,
    connection::Connection,
    protocol::{
        Event,
        xproto::*,
        render::*,
    },
    rust_connection::RustConnection,
};
use xmodmap::{KeyTable, Modifier, KeySym};
use std::process::Command;
use matsuba::{
    error::{BoxResult, SimpleError},
    config::{MUHENKAN_KEY, HENKAN_KEY},
};

use super::db;
use super::db::DBConnection;
use super::converter::{State, Converter};
use super::xutils::{create_face, create_glyph, draw_text, x_to_xmodmap_modifier, xmodmap_to_x_modifier};

pub struct XSession {
    conn: RustConnection,
    screen_num: usize,
    completion_box: Option<Window>,
    completion_box_text: String,
    conversion_options: Vec<String>,
    current_conversion: usize,
    keytable: KeyTable,
    converter: Converter,
    running: bool,
    henkan: bool,
}

// TODO replace these
const TEXT_WIDTH: u32 = 10;
const TEXT_HEIGHT: u32 = 20;

impl XSession {

    pub fn new() -> BoxResult<XSession> {

        let (conn, screen_num) = x11rb::connect(None)?;

        let keytable = KeyTable::new()?;
        let converter = Converter::new();

        Ok(XSession {
            conn: conn,
            screen_num: screen_num,
            completion_box: None,
            completion_box_text: String::new(),
            conversion_options: Vec::new(),
            current_conversion: 0, // TODO replace this with an iterator prob
            keytable: keytable,
            converter: converter,
            running: true,
            henkan: true,
        })

    }

    fn screen(&self) -> &Screen {
        &self.conn.setup().roots[self.screen_num]
    }

    pub fn run(&mut self) -> BoxResult<()> {

        self.configure_root()?;
        while self.is_running() {
            self.render_completion_box()?;
            self.conn.flush()?;

            let event = self.conn.wait_for_event()?;
            self.handle_event(&event)?;
        }
        Ok(())
    }

    fn configure_root(&self) -> BoxResult<()> {

        // append to root window attributes
        let attrs = self.conn.get_window_attributes(self.screen().root)?.reply()?;
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(attrs.your_event_mask|EventMask::SUBSTRUCTURE_NOTIFY); // TODO this might need to be attrs.all_event_masks
        self.conn.change_window_attributes(self.screen().root, &values_list)?.check()?;

        self.grab_keyboard()?;

        Ok(())
    }

    fn grab_keyboard(&self) -> BoxResult<()> {

        // grab user keypresses
        let grab_status = self.conn.grab_keyboard(false, self.screen().root, CURRENT_TIME, GrabMode::ASYNC, GrabMode::ASYNC)?.reply()?;
        if grab_status.status != GrabStatus::SUCCESS {
            return Err(Box::new(SimpleError::new("error grabbing keyboard")));
        }
        Ok(())
    }

    fn ungrab_keyboard(&self) -> BoxResult<()> {
        self.conn.ungrab_keyboard(CURRENT_TIME)?.check()?;
        // the only key we still want to grab is the muhenkan key

        let (henkan_mod, henkan_keysym) = self.keytable.get_key(HENKAN_KEY)?;
        let henkan_mod = xmodmap_to_x_modifier(henkan_mod);
        self.conn.grab_key(true, self.screen().root, henkan_mod, henkan_keysym, GrabMode::ASYNC, GrabMode::ASYNC)?.check()?;
        Ok(())
    }

    pub fn font_init(&self) -> BoxResult<()> {

        let pictformats = query_pict_formats(&self.conn)?.reply()?;
        // TODO hardcoded pictformat for now
        let format = pictformats.formats.iter().find(|f| f.id == 35).unwrap();
        for pf in pictformats.formats.iter() {
            println!("{:?}", pf);
        }
        // println!("{:?}", format);

        let face = create_face("sans")?;

        // xcb glyph init
        let gsid = self.conn.generate_id()?;
        create_glyph_set(&self.conn, gsid, format.id)?.check()?;
        create_glyph(&self.conn, &face, gsid, '???')?;
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> BoxResult<()> {

        match event {
            Event::KeyPress(event) => {
                self.handle_keypress(event)?;
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_keypress(&mut self, event: &KeyPressEvent) -> BoxResult<()> {

        // extract key press info
        let modifier = x_to_xmodmap_modifier(event.state);
        let keysym = self.keytable.get_keysym(modifier, event.detail);
        if keysym.is_err() { return Ok(()); }
        let keysym = keysym.unwrap();

        // key bindings that are active regardless if we are converting
        match keysym {
            MUHENKAN_KEY => {
                self.henkan = false;
                self.ungrab_keyboard()?;
                return Ok(());
            },
            HENKAN_KEY => {
                self.henkan = true;
                self.grab_keyboard()?;
                return Ok(());
            }
            _ => {}
        };

        // exit if we are not converting
        if !self.henkan {
            return Ok(());
        }

        // keybindings that are active only when we are converted
        match keysym {
            KeySym::KEY_GRAVE => {
                // temp way to exit
                self.running = false;
            }
            KeySym::KEY_RETURN => {
                self.output_conversion()?;
            }
            KeySym::KEY_BACKSPACE => {
                self.converter.del_char();
                self.completion_box_text = self.converter.output.clone();
                println!("{}", self.completion_box_text);
            }
            KeySym::KEY_ESCAPE => {

                // cancel out of conversion
                self.conversion_options.clear();
                self.current_conversion = 0;

                // bring back raw kana
                self.completion_box_text = self.converter.output.clone();
                println!("{}", self.completion_box_text);

            }
            KeySym::KEY_TAB => {

                if self.conversion_options.len() > 0 {

                    // conversion already done, cycle through options
                    self.current_conversion = (self.current_conversion+1) % (self.conversion_options.len());

                } else {

                    // conversion not done, populate conversion options list
                    let db_conn = db::get_connection()?;
                    let kana = &self.converter.output;
                    let converted = db::search(&db_conn, kana)?;

                    for entry in converted {
                        self.conversion_options.push(entry.k_ele);
                    }

                    // always push exactly what we typed
                    self.conversion_options.push(kana.clone());

                    // set current to beginning
                    self.current_conversion = 0;
                }
                self.completion_box_text = self.conversion_options.get(self.current_conversion).unwrap().to_string();
                println!("{}", self.completion_box_text);
            }
            _ => {

                // if start typing with conversion open, instantly accept
                if self.conversion_options.len() > 0 {
                    self.output_conversion()?;
                }

                // reopen completion window if closed
                if self.completion_box.is_none() {
                    // TODO get position that is typing
                    self.create_completion_box((event.event_x, event.event_y))?;
                }

                let ch = keysym.as_char();
                if ch.is_none() { return Ok(()); }
                self.converter.input_char(ch.unwrap());
                self.completion_box_text = self.converter.output.clone();
                println!("{}", self.completion_box_text);
            }
        }

        Ok(())
    }

    fn output_conversion(&mut self) -> BoxResult<()> {

        println!("accept {}", self.completion_box_text);

        // find currently focused window
        let focused_win = self.conn.get_input_focus()?.reply()?.focus;

        // TODO this is ugly and also cheating lmao
        self.ungrab_keyboard()?;
        let result = Command::new("xdotool")
            .args(["type", "--window", &focused_win.to_string(), "--delay", "200", &self.completion_box_text])
            .output();
        if result.is_err() {
            println!("errored {:?}", result);
        }
        self.grab_keyboard()?;
        // self.send_keypress(focused_win, KeySym::KEY_A)?;

        self.converter.accept();

        // clear conversion options
        self.conversion_options.clear();
        self.current_conversion = 0;

        // close completion box when done
        self.destroy_completion_box()?;
        self.completion_box = None;
        self.completion_box_text = String::new();

        Ok(())
    }

    pub fn create_completion_box(&mut self, position: (i16, i16)) -> BoxResult<()> {

        // create completion box window
        let win = self.conn.generate_id()?;
        let values_list = CreateWindowAux::default()
            .background_pixel(self.screen().white_pixel)
            .override_redirect(1); // make window manager ignore the window
        self.conn.create_window(
            0, // draw on top of everything
            win,
            self.screen().root,
            position.0,
            position.1,
            TEXT_WIDTH as u16,
            TEXT_HEIGHT as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            self.screen().root_visual,
            &values_list,
        )?;

        self.completion_box = Some(win);

        Ok(())
    }

    // pub fn render_completion_box(&self, position: (i16, i16), text: &str) -> BoxResult<()> {
    fn render_completion_box(&self) -> BoxResult<()> {

        if self.completion_box.is_none() { return Ok(()); }
        let win = self.completion_box.unwrap();
        
        // resize window to fit text
        let values_list = ConfigureWindowAux::default()
            .width(TEXT_WIDTH*(self.completion_box_text.len() as u32));
        self.conn.configure_window(win, &values_list)?;

        draw_text(&self.conn, &self.screen(), win, TEXT_WIDTH as i16, TEXT_HEIGHT as i16, "mtx", &self.completion_box_text)?;
        self.conn.map_window(win)?;

        Ok(())
    }

    fn destroy_completion_box(&mut self) -> BoxResult<()> {
        if self.completion_box.is_none() { return Ok(()); }
        self.conn.destroy_window(self.completion_box.unwrap())?;
        Ok(())
    }

    fn send_keypress(&self, win: Window, keysym: KeySym) -> BoxResult<()> {
    // fn send_keypress(&self, win: Window, keycode: u8, modifier: u16) -> BoxResult<()> {

        // get keycode from keymask
        let (modifier, keycode) = self.keytable.get_key(keysym)?;
        let modifier = xmodmap_to_x_modifier(modifier);

        let press_event = KeyPressEvent {
            response_type: KEY_PRESS_EVENT,
            detail: keycode,
            sequence: 0,
            time: CURRENT_TIME,
            root: self.screen().root,
            event: self.screen().root,
            child: win,
            root_x: 1,
            root_y: 1,
            event_x: 1,
            event_y: 1,
            state: modifier,
            same_screen: true,
        };
        self.conn.send_event(false, win, EventMask::KEY_PRESS, press_event)?.check()?;

        let release_event = KeyReleaseEvent {
            response_type: KEY_RELEASE_EVENT,
            detail: keycode,
            sequence: 0,
            time: CURRENT_TIME,
            root: self.screen().root,
            event: self.screen().root,
            child: win,
            root_x: 1,
            root_y: 1,
            event_x: 1,
            event_y: 1,
            state: modifier,
            same_screen: true,
        };
        self.conn.send_event(false, win, EventMask::KEY_RELEASE, release_event)?.check()?;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

}

