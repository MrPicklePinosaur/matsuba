// config file for matsuba
use config::{Config, ConfigError, File};
use serde::Deserialize;
use winit::event::VirtualKeyCode;

pub static CACHE_DIR: &str = ".cache/matsuba"; // where the database file goes
pub const MUHENKAN_KEY: VirtualKeyCode = VirtualKeyCode::Key9;
pub const HENKAN_KEY: VirtualKeyCode = VirtualKeyCode::Key0;

#[derive(Debug, Deserialize)]
pub struct Settings {
    // pub keys: KeyMap
    pub cache_dir: String,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub listen_address: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cache_dir: ".cache/matsuba".into(),
            server: Server::default(),
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            listen_address: "[::1]:10000".into(),
        }
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
            .add_source(File::with_name("matsuba.toml"))
            .build()?;

        conf.try_deserialize()
    }
}
