use crate::config::HENKAN_KEY;
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
    pub fn new() -> Result<XSession, Box<dyn std::error::Error>> {
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

    pub fn handle_keypress(
        &self,
        mut handler: impl FnMut(KeySym, Modifier),
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.flush()?;

        if let Some(event) = self.conn.poll_for_event()? {
            match event {
                Event::KeyPress(event) => {
                    // extract key press info
                    let modifier = x_to_xmodmap_modifier(event.state);
                    let keysym = self.keytable.get_keysym(modifier.clone(), event.detail)?;

                    handler(keysym, modifier);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn configure_root(&self) -> Result<(), Box<dyn std::error::Error>> {
        // append to root window attributes
        let attrs = get_window_attributes(&self.conn, self.screen().root)?.reply()?;
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(attrs.your_event_mask | EventMask::SUBSTRUCTURE_NOTIFY); // TODO this might need to be attrs.all_event_masks
        change_window_attributes(&self.conn, self.screen().root, &values_list)?.check()?;

        self.grab_keyboard()?;

        Ok(())
    }

    pub fn grab_keyboard(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (henkan_mod, henkan_keysym) = self.keytable.get_key(HENKAN_KEY)?;
        // ungrab_key(&self.conn, henkan_keysym, self.screen().root, ModMask::ANY)?.check()?;

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

    pub fn ungrab_keyboard(&self) -> Result<(), Box<dyn std::error::Error>> {
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

fn x_to_xmodmap_modifier(state: KeyButMask) -> Modifier {
    if u16::from(state) & u16::from(KeyButMask::SHIFT) == 0 {
        Modifier::Key
    } else {
        Modifier::ShiftKey
    }
}

fn xmodmap_to_x_modifier(modifier: Modifier) -> ModMask {
    match modifier {
        Modifier::Key => ModMask::from(0u8),
        Modifier::ShiftKey => ModMask::SHIFT,
        _ => ModMask::from(0u8), // TODO maybe should return error?
    }
}
