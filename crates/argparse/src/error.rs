
use std::fmt;

pub type BoxResult<T> = Result<T,Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Error {
    pub msg: String
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_string()
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

