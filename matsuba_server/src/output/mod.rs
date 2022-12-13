//! Handles sending output to active window
//!
//! This module is very system dependent. Many implementations are required for various platforms

use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum OutputError {
    Xdotool(String),
    NoOutputMethod,
}

impl Error for OutputError {}
impl Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xdotool(e) => write!(f, "xdotool error: {}", e),
            Self::NoOutputMethod => write!(f, "no output method defined"),
        }
    }
}

pub fn output(out: &str) -> Result<(), OutputError> {
    if cfg!(feature = "x11") {
        output_x11(out)?;
    } else {
        return Err(OutputError::NoOutputMethod);
    }

    Ok(())
}

#[cfg(feature = "x11")]
fn output_x11(out: &str) -> Result<(), OutputError> {
    use std::process::Command;
    Command::new("xdotool")
        .args(["type", "--window", "20", out])
        .output()
        .map_err(|e| OutputError::Xdotool(e.to_string()))?;

    Ok(())
}
