
use std::fmt;

pub type BoxResult<T> = Result<T,Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
    InvalidFlag,
    MissingFlagValue,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO make actual error messages
        write!(f, "error")
    }
}

