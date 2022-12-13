use pino_xmodmap::{KeySym, KeyTable, Modifier};
use std::process::Command;
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use x11rb::{
    connection::Connection,
    protocol::{render::*, xproto::*, Event},
    rust_connection::RustConnection,
    CURRENT_TIME,
};

pub const HENKAN_KEY: KeySym = KeySym::KEY_0;

#[derive(Debug)]
pub enum XorgError {
    ConnectionFailure,
    Keytable,
    KeyboardGrabFailed,
}

impl Error for XorgError {}
impl Display for XorgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionFailure => write!(f, "could not connect to x server"),
            Self::Keytable => write!(f, "failed initializing key table"),
            Self::KeyboardGrabFailed => write!(f, "could not grab keyboard"),
        }
    }
}

pub struct XSession {
    conn: RustConnection,
    screen_num: usize,
    keytable: KeyTable,
}

impl XSession {
    pub fn new() -> Result<XSession, XorgError> {
        let (conn, screen_num) = x11rb::connect(None).map_err(|_| XorgError::ConnectionFailure)?;

        let keytable = KeyTable::new().map_err(|_| XorgError::Keytable)?;

        Ok(XSession {
            conn,
            screen_num,
            keytable,
        })
    }

    fn screen(&self) -> &Screen {
        &self.conn.setup().roots[self.screen_num]
    }

    /*
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.configure_root()?;
        while self.is_running() {
            self.render_completion_box()?;
            self.conn.flush()?;

            let event = self.conn.wait_for_event()?;
            self.handle_event(&event)?;
        }
        Ok(())
    }
    */

    fn configure_root(&self) -> Result<(), Box<dyn std::error::Error>> {
        // append to root window attributes
        let attrs = get_window_attributes(&self.conn, self.screen().root)?.reply()?;
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(attrs.your_event_mask | EventMask::SUBSTRUCTURE_NOTIFY); // TODO this might need to be attrs.all_event_masks
        change_window_attributes(&self.conn, self.screen().root, &values_list)?.check()?;

        self.grab_keyboard()?;

        Ok(())
    }

    fn grab_keyboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        // grab user keypresses
        let grab_status = grab_keyboard(
            &self.conn,
            false,
            self.screen().root,
            CURRENT_TIME,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        )?
        .reply()?;
        if grab_status.status != GrabStatus::SUCCESS {
            return Err(Box::new(XorgError::KeyboardGrabFailed));
        }
        Ok(())
    }

    fn ungrab_keyboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        ungrab_keyboard(&self.conn, CURRENT_TIME)?.check()?;
        // the only key we still want to grab is the muhenkan key

        let (henkan_mod, henkan_keysym) = self.keytable.get_key(HENKAN_KEY)?;
        let henkan_mod = xmodmap_to_x_modifier(henkan_mod);
        grab_key(
            &self.conn,
            true,
            self.screen().root,
            henkan_mod,
            henkan_keysym,
            GrabMode::ASYNC,
            GrabMode::ASYNC,
        )?
        .check()?;
        Ok(())
    }
}

fn xmodmap_to_x_modifier(modifier: Modifier) -> ModMask {
    match modifier {
        Modifier::Key => ModMask::ANY,
        Modifier::ShiftKey => ModMask::SHIFT,
        _ => ModMask::ANY, // TODO maybe should return error?
    }
}
