
use std::process::Command;
use std::collections::HashMap;

use super::keysym::{KeySym, FromStr};
use super::error::{SimpleError, BoxResult};

type KeyCode = (u16,u8);
type KeyTable = HashMap<KeyCode, KeySym>;

// requires that user has xmodmap program installed
pub fn load_xmodmap() -> BoxResult<KeyTable> {
    
    let mut keytable = HashMap::new();

    let output = Command::new("xmodmap").arg("-pke").output()?;
    let raw_xmodmap = String::from_utf8(output.stdout)?;

    for l in raw_xmodmap.lines() {
        let mut split = l.split_ascii_whitespace();

        assert_eq!(Some("keycode"), split.next());
        let keycode = split
            .next().ok_or(SimpleError::new("error reading keycode"))?
            .parse::<u8>()?;
        assert_eq!(Some("="), split.next());

        // TODO handle case where next() fails in a better way
        let a = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
        let b = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
        keytable.insert((0,keycode), a);
        keytable.insert((1,keycode), b);
    }

    Ok(keytable)
}

