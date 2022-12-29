// config file for matsuba
use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use pino_xmodmap::KeySym;
use serde::{de::Visitor, Deserialize};

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

/*
#[derive(Debug, Deserialize)]
pub struct KeyMap {
    /// Toggle conversion mode on
    pub henkan: VirtualKeyCode,
    /// Toggle conversion mode off
    pub muhenkan: VirtualKeyCode,
    /// Accept currently selected conversion
    pub accept: VirtualKeyCode,
    /// Abort currently selected conversions
    pub cancel: VirtualKeyCode,
    /// Cycle to the next conversion
    pub next_conversion: VirtualKeyCode,
    /// Cycle to the previous conversion
    pub prev_conversion: VirtualKeyCode,
}
*/

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
}
