// config file for matsuba
use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use pino_xmodmap::{FromStr, KeySym, KeyTable, Modifier};
use serde::{de::Visitor, Deserialize};
use x11rb::protocol::xproto::KeyButMask;

pub const HENKAN_KEY: KeySym = KeySym::KEY_0;
pub const MUHENKAN_KEY: KeySym = KeySym::KEY_9;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::load().expect("Issue parsing config");
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    // pub keys: KeyMap
    pub server: Server,
    pub theme: Theme,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub listen_address: String,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    /// Background color of the window
    pub bg: Color,
    /// Text color
    pub fg: Color,
    /// Background color for the currently selected completion
    pub selected_bg: Color,
    /// Text color for the currently selected completion
    pub selected_fg: Color,
    /// Background color for the completion
    pub completion_bg: Color,
    /// Text color for the completion
    pub completion_fg: Color,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub cache_dir: String,
}

#[derive(Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_hex(hexstring: &str) -> Option<Self> {
        // strip leading # if provided
        let hexstring = hexstring.trim_start_matches('#');

        if hexstring.len() != 6 && hexstring.len() != 8 {
            return None;
        }

        let mut res = [0.0, 0.0, 0.0, 1.0];
        for i in 0..(hexstring.len() / 2) {
            let color_val = i64::from_str_radix(&hexstring[i..i + 2], 16);
            if let Ok(color_val) = color_val {
                res[i] = (color_val as f32 / 255.0).clamp(0.0, 1.0);
            }
        }

        Some(Self {
            r: res[0],
            g: res[1],
            b: res[2],
            a: res[3],
        })
    }

    pub fn as_slice_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn as_slice_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

struct ColorVisitor;
impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a hex color code (etc #FFFFFF or #550055FF)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Color::from_hex(value).ok_or(serde::de::Error::custom("error parsing hex string"))
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    /// Toggle conversion mode on
    pub henkan: Keybinding,
    /// Toggle conversion mode off
    pub muhenkan: Keybinding,
    /// Accept currently selected conversion
    pub accept: Keybinding,
    /// Abort currently selected conversions
    pub cancel: Keybinding,
    /// Cycle to the next conversion
    pub next_conversion: Keybinding,
    /// Cycle to the previous conversion
    pub prev_conversion: Keybinding,
}

#[derive(Debug)]
pub enum KeybindingError {
    TooShort,
    InvalidModifier(String),
    InvalidKey(String),
}

impl std::error::Error for KeybindingError {}
impl std::fmt::Display for KeybindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooShort => write!(f, "keybinding string too short"),
            Self::InvalidModifier(m) => write!(f, "invalid modifier recieved {}", m),
            Self::InvalidKey(k) => write!(f, "invalid key recieved {}", k),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Keybinding {
    pub mod_mask: KeyButMask,
    pub key: KeySym,
}

impl Keybinding {
    pub fn from_str(key_str: &str) -> Result<Self, KeybindingError> {
        let mut mods = key_str
            .split("-")
            .filter(|&s| !s.is_empty())
            .collect::<Vec<_>>();

        if mods.is_empty() {
            return Err(KeybindingError::TooShort);
        }
        let raw_key = mods.pop().unwrap();
        let key = KeySym::from_str(raw_key)
            .map_err(|_| KeybindingError::InvalidKey(raw_key.to_owned()))?;

        let mut mod_mask = KeyButMask::default();

        for modifier in mods {
            match modifier {
                "M" => {
                    mod_mask = mod_mask | KeyButMask::MOD1;
                }
                "S" => {
                    mod_mask = mod_mask | KeyButMask::SHIFT;
                }
                "C" => {
                    mod_mask = mod_mask | KeyButMask::CONTROL;
                }
                _ => {
                    return Err(KeybindingError::InvalidModifier(modifier.to_owned()));
                }
            }
        }

        Ok(Self { mod_mask, key })
    }
}

impl<'de> Deserialize<'de> for Keybinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(KeybindingVisitor)
    }
}

struct KeybindingVisitor;
impl<'de> Visitor<'de> for KeybindingVisitor {
    type Value = Keybinding;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A keybinding string (ex M-a C-S-x)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Keybinding::from_str(value).map_err(|e| {
            serde::de::Error::custom(format!(
                "error parsing keybinding string: {}",
                e.to_string()
            ))
        })
    }
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("matsuba_default.toml"))
            // .add_source(File::with_name("matsuba.toml"))
            .build()?;

        conf.try_deserialize()
    }
}

#[cfg(test)]
mod tests {

    use x11rb::protocol::xproto::KeyButMask;

    use crate::config::Settings;

    #[test]
    fn simple() {
        let settings = Settings::load();
        println!("{:?}", settings);
        assert!(settings.is_ok());
    }

    #[test]
    fn color_test() {
        use super::Color;

        println!("{:?}", Color::from_hex("#ABCDEF"));
    }

    #[test]
    fn keybinding_test() {
        use super::Keybinding;

        assert_eq!(
            Keybinding::from_str("M-a").unwrap(),
            Keybinding {
                mod_mask: KeyButMask::MOD1,
                key: pino_xmodmap::KeySym::KEY_a
            }
        );
        assert_eq!(
            Keybinding::from_str("C-S-a").unwrap(),
            Keybinding {
                mod_mask: KeyButMask::CONTROL | KeyButMask::SHIFT,
                key: pino_xmodmap::KeySym::KEY_a
            }
        );
    }
}
