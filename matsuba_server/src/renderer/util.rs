use std::{error::Error, fmt::Display};

use winit::event::{ModifiersState, VirtualKeyCode};

#[derive(Clone, Copy)]
pub struct Key(pub VirtualKeyCode, pub ModifiersState);

#[derive(Debug)]
pub struct TryFromKeyError;
impl Error for TryFromKeyError {}
impl Display for TryFromKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not convert from key to char")
    }
}

impl TryFrom<Key> for char {
    type Error = TryFromKeyError;
    fn try_from(value: Key) -> Result<Self, Self::Error> {
        let Key(k, m) = value;
        let byte = match k {
            VirtualKeyCode::A if !m.shift() => 0x61,
            VirtualKeyCode::B if !m.shift() => 0x62,
            VirtualKeyCode::C if !m.shift() => 0x63,
            VirtualKeyCode::D if !m.shift() => 0x64,
            VirtualKeyCode::E if !m.shift() => 0x65,
            VirtualKeyCode::F if !m.shift() => 0x66,
            VirtualKeyCode::G if !m.shift() => 0x67,
            VirtualKeyCode::H if !m.shift() => 0x68,
            VirtualKeyCode::I if !m.shift() => 0x69,
            VirtualKeyCode::J if !m.shift() => 0x6a,
            VirtualKeyCode::K if !m.shift() => 0x6b,
            VirtualKeyCode::L if !m.shift() => 0x6c,
            VirtualKeyCode::M if !m.shift() => 0x6d,
            VirtualKeyCode::N if !m.shift() => 0x6e,
            VirtualKeyCode::O if !m.shift() => 0x6f,
            VirtualKeyCode::P if !m.shift() => 0x70,
            VirtualKeyCode::Q if !m.shift() => 0x71,
            VirtualKeyCode::R if !m.shift() => 0x72,
            VirtualKeyCode::S if !m.shift() => 0x73,
            VirtualKeyCode::T if !m.shift() => 0x74,
            VirtualKeyCode::U if !m.shift() => 0x75,
            VirtualKeyCode::V if !m.shift() => 0x76,
            VirtualKeyCode::W if !m.shift() => 0x77,
            VirtualKeyCode::X if !m.shift() => 0x78,
            VirtualKeyCode::Y if !m.shift() => 0x79,
            VirtualKeyCode::Z if !m.shift() => 0x7a,
            VirtualKeyCode::A if m.shift() => 0x41,
            VirtualKeyCode::B if m.shift() => 0x42,
            VirtualKeyCode::C if m.shift() => 0x43,
            VirtualKeyCode::D if m.shift() => 0x44,
            VirtualKeyCode::E if m.shift() => 0x45,
            VirtualKeyCode::F if m.shift() => 0x46,
            VirtualKeyCode::G if m.shift() => 0x47,
            VirtualKeyCode::H if m.shift() => 0x48,
            VirtualKeyCode::I if m.shift() => 0x49,
            VirtualKeyCode::J if m.shift() => 0x4a,
            VirtualKeyCode::K if m.shift() => 0x4b,
            VirtualKeyCode::L if m.shift() => 0x4c,
            VirtualKeyCode::M if m.shift() => 0x4d,
            VirtualKeyCode::N if m.shift() => 0x4e,
            VirtualKeyCode::O if m.shift() => 0x4f,
            VirtualKeyCode::P if m.shift() => 0x50,
            VirtualKeyCode::Q if m.shift() => 0x51,
            VirtualKeyCode::R if m.shift() => 0x52,
            VirtualKeyCode::S if m.shift() => 0x53,
            VirtualKeyCode::T if m.shift() => 0x54,
            VirtualKeyCode::U if m.shift() => 0x55,
            VirtualKeyCode::V if m.shift() => 0x56,
            VirtualKeyCode::W if m.shift() => 0x57,
            VirtualKeyCode::X if m.shift() => 0x58,
            VirtualKeyCode::Y if m.shift() => 0x59,
            VirtualKeyCode::Z if m.shift() => 0x5a,
            _ => return Err(TryFromKeyError),
        };

        char::from_u32(byte).ok_or(TryFromKeyError)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(c) = char::try_from(*self) {
            write!(f, "{}", c)?;
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}

// impl Deserialize for Key {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: serde::Deserializer<'de>
//     {
//
//     }
// }
