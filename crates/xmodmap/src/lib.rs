
use std::process::Command;
use std::collections::HashMap;
use std::fmt;
pub use std::str::FromStr;

#[derive(std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash, Clone)]
pub enum Modifier {
    Key,
    ShiftKey,
    ModeSwitchKey,
    ModeSwitchShiftKey,
    ISOLevel3ShiftKey,
    ISOLevel3ShiftShiftKey
}

pub type KeyCode = u8;
pub type Key = (Modifier,KeyCode);

pub struct KeyTable {
    key_to_keysym: HashMap<Key,KeySym>,
    keysym_to_key: HashMap<KeySym,Key>,
}

#[derive(Debug)]
pub enum Error {
    XmodmapRunError,
    InvalidFormat,
    NonExistentKeyCode,
    NonExistentKeySym,
}

impl std::error::Error for Error { }
impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::XmodmapRunError => write!(f, "{}", "could not run xmodmap command, do you have it installed?"),
            Error::InvalidFormat => write!(f, "{}", "invalid xmodmap format"),
            Error::NonExistentKeyCode => write!(f, "{}", "non-existent keycode"),
            Error::NonExistentKeySym => write!(f, "{}", "non-existent keysym"),
        }
    }

}

impl KeyTable {

    // requires that user has xmodmap program installed
    pub fn new() -> Result<Self, Error> {
        
        let mut key_to_keysym: HashMap<Key,KeySym> = HashMap::new();
        let mut keysym_to_key: HashMap<KeySym,Key> = HashMap::new();

        let output = Command::new("xmodmap").arg("-pke")
            .output()
            .or(Err(Error::XmodmapRunError))?;
        let raw_xmodmap = String::from_utf8(output.stdout)
            .or(Err(Error::XmodmapRunError))?;

        for l in raw_xmodmap.lines() {
            let mut split = l.split_ascii_whitespace();

            assert_eq!(Some("keycode"), split.next());
            let keycode = split
                .next().ok_or(Error::InvalidFormat)?
                .parse::<u8>().or(Err(Error::InvalidFormat))?;
            assert_eq!(Some("="), split.next());

            // TODO handle case where next() fails in a better way
            let a = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
            let b = KeySym::from_str(split.next().unwrap_or("")).unwrap_or(KeySym::KEY_NONE);
            key_to_keysym.insert((Modifier::Key,keycode), a.clone());
            keysym_to_key.insert(a, (Modifier::Key,keycode));
            key_to_keysym.insert((Modifier::ShiftKey,keycode), b.clone());
            keysym_to_key.insert(b, (Modifier::Key,keycode));
        }

        Ok(KeyTable{
            key_to_keysym: key_to_keysym,
            keysym_to_key: keysym_to_key,
        })
    }

    pub fn get_keysym(&self, modifier: Modifier, code: KeyCode) -> Result<KeySym, Error> {
        match self.key_to_keysym.get(&(modifier,code)) {
            Some(k) => Ok(k.clone()),
            None => Err(Error::NonExistentKeyCode),
        }
    }

    pub fn get_key(&self, keysym: KeySym) -> Result<Key, Error> {
        match self.keysym_to_key.get(&keysym) {
            Some(k) => Ok(k.clone()),
            None => Err(Error::NonExistentKeySym),
        }
    }
}

pub static ALL_LOWER_CASE: &'static [KeySym] = &[KeySym::KEY_a,KeySym::KEY_b,KeySym::KEY_c,KeySym::KEY_d,KeySym::KEY_e,KeySym::KEY_f,KeySym::KEY_g,KeySym::KEY_h,KeySym::KEY_i,KeySym::KEY_j,KeySym::KEY_k,KeySym::KEY_l,KeySym::KEY_m,KeySym::KEY_n,KeySym::KEY_o,KeySym::KEY_p,KeySym::KEY_q,KeySym::KEY_r,KeySym::KEY_s,KeySym::KEY_t,KeySym::KEY_u,KeySym::KEY_v,KeySym::KEY_w,KeySym::KEY_x,KeySym::KEY_y,KeySym::KEY_z];
pub static ALL_UPPER_CASE: &'static [KeySym] = &[KeySym::KEY_A,KeySym::KEY_B,KeySym::KEY_C,KeySym::KEY_D,KeySym::KEY_E,KeySym::KEY_F,KeySym::KEY_G,KeySym::KEY_H,KeySym::KEY_I,KeySym::KEY_J,KeySym::KEY_K,KeySym::KEY_L,KeySym::KEY_M,KeySym::KEY_N,KeySym::KEY_O,KeySym::KEY_P,KeySym::KEY_Q,KeySym::KEY_R,KeySym::KEY_S,KeySym::KEY_T,KeySym::KEY_U,KeySym::KEY_V,KeySym::KEY_W,KeySym::KEY_X,KeySym::KEY_Y,KeySym::KEY_Z];

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum KeySym {
    KEY_NONE,
    KEY_a,
    KEY_b,
    KEY_c,
    KEY_d,
    KEY_e,
    KEY_f,
    KEY_g,
    KEY_h,
    KEY_i,
    KEY_j,
    KEY_k,
    KEY_l,
    KEY_m,
    KEY_n,
    KEY_o,
    KEY_p,
    KEY_q,
    KEY_r,
    KEY_s,
    KEY_t,
    KEY_u,
    KEY_v,
    KEY_w,
    KEY_x,
    KEY_y,
    KEY_z,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_SPACE,
    KEY_RETURN,
    KEY_BACKSPACE,
}

impl FromStr for KeySym {

    type Err = ();

    fn from_str(input: &str) -> Result<KeySym, Self::Err> {
        match input {
            "a" => Ok(KeySym::KEY_a),
            "b" => Ok(KeySym::KEY_b),
            "c" => Ok(KeySym::KEY_c),
            "d" => Ok(KeySym::KEY_d),
            "e" => Ok(KeySym::KEY_e),
            "f" => Ok(KeySym::KEY_f),
            "g" => Ok(KeySym::KEY_g),
            "h" => Ok(KeySym::KEY_h),
            "i" => Ok(KeySym::KEY_i),
            "j" => Ok(KeySym::KEY_j),
            "k" => Ok(KeySym::KEY_k),
            "l" => Ok(KeySym::KEY_l),
            "m" => Ok(KeySym::KEY_m),
            "n" => Ok(KeySym::KEY_n),
            "o" => Ok(KeySym::KEY_o),
            "p" => Ok(KeySym::KEY_p),
            "q" => Ok(KeySym::KEY_q),
            "r" => Ok(KeySym::KEY_r),
            "s" => Ok(KeySym::KEY_s),
            "t" => Ok(KeySym::KEY_t),
            "u" => Ok(KeySym::KEY_u),
            "v" => Ok(KeySym::KEY_v),
            "w" => Ok(KeySym::KEY_w),
            "x" => Ok(KeySym::KEY_x),
            "y" => Ok(KeySym::KEY_y),
            "z" => Ok(KeySym::KEY_z),
            "A" => Ok(KeySym::KEY_A),
            "B" => Ok(KeySym::KEY_B),
            "C" => Ok(KeySym::KEY_C),
            "D" => Ok(KeySym::KEY_D),
            "E" => Ok(KeySym::KEY_E),
            "F" => Ok(KeySym::KEY_F),
            "G" => Ok(KeySym::KEY_G),
            "H" => Ok(KeySym::KEY_H),
            "I" => Ok(KeySym::KEY_I),
            "J" => Ok(KeySym::KEY_J),
            "K" => Ok(KeySym::KEY_K),
            "L" => Ok(KeySym::KEY_L),
            "M" => Ok(KeySym::KEY_M),
            "N" => Ok(KeySym::KEY_N),
            "O" => Ok(KeySym::KEY_O),
            "P" => Ok(KeySym::KEY_P),
            "Q" => Ok(KeySym::KEY_Q),
            "R" => Ok(KeySym::KEY_R),
            "S" => Ok(KeySym::KEY_S),
            "T" => Ok(KeySym::KEY_T),
            "U" => Ok(KeySym::KEY_U),
            "V" => Ok(KeySym::KEY_V),
            "W" => Ok(KeySym::KEY_W),
            "X" => Ok(KeySym::KEY_X),
            "Y" => Ok(KeySym::KEY_Y),
            "Z" => Ok(KeySym::KEY_Z),
            "space" => Ok(KeySym::KEY_SPACE),
            "Return" => Ok(KeySym::KEY_RETURN),
            "BackSpace" => Ok(KeySym::KEY_BACKSPACE),
            "NoSymbol" => Ok(KeySym::KEY_NONE),
            _ => Err(()),
        }
    }
}

impl KeySym {
    pub fn as_char(&self) -> Option<char> {
        char::from_u32(
            (match self {
                KeySym::KEY_A => 0x41,
                KeySym::KEY_B => 0x42,
                KeySym::KEY_C => 0x43,
                KeySym::KEY_D => 0x44,
                KeySym::KEY_E => 0x45,
                KeySym::KEY_F => 0x46,
                KeySym::KEY_G => 0x47,
                KeySym::KEY_H => 0x48,
                KeySym::KEY_I => 0x49,
                KeySym::KEY_J => 0x4a,
                KeySym::KEY_K => 0x4b,
                KeySym::KEY_L => 0x4c,
                KeySym::KEY_M => 0x4d,
                KeySym::KEY_N => 0x4e,
                KeySym::KEY_O => 0x4f,
                KeySym::KEY_P => 0x50,
                KeySym::KEY_Q => 0x51,
                KeySym::KEY_R => 0x52,
                KeySym::KEY_S => 0x53,
                KeySym::KEY_T => 0x54,
                KeySym::KEY_U => 0x55,
                KeySym::KEY_V => 0x56,
                KeySym::KEY_W => 0x57,
                KeySym::KEY_X => 0x58,
                KeySym::KEY_Y => 0x59,
                KeySym::KEY_Z => 0x5a,
                KeySym::KEY_a => 0x61,
                KeySym::KEY_b => 0x62,
                KeySym::KEY_c => 0x63,
                KeySym::KEY_d => 0x64,
                KeySym::KEY_e => 0x65,
                KeySym::KEY_f => 0x66,
                KeySym::KEY_g => 0x67,
                KeySym::KEY_h => 0x68,
                KeySym::KEY_i => 0x69,
                KeySym::KEY_j => 0x6a,
                KeySym::KEY_k => 0x6b,
                KeySym::KEY_l => 0x6c,
                KeySym::KEY_m => 0x6d,
                KeySym::KEY_n => 0x6e,
                KeySym::KEY_o => 0x6f,
                KeySym::KEY_p => 0x70,
                KeySym::KEY_q => 0x71,
                KeySym::KEY_r => 0x72,
                KeySym::KEY_s => 0x73,
                KeySym::KEY_t => 0x74,
                KeySym::KEY_u => 0x75,
                KeySym::KEY_v => 0x76,
                KeySym::KEY_w => 0x77,
                KeySym::KEY_x => 0x78,
                KeySym::KEY_y => 0x79,
                KeySym::KEY_z => 0x7a,
                _ => 0x00,
            } as u32)
        )
    }
}

