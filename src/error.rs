use std::{
    error,
    fmt::{self, Formatter},
    io,
    path::PathBuf,
    result,
};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadIo(io::Error),
    InvalidPe(peview::error::Error),
    NonExistendPath(PathBuf),
    InvalidExtension(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadIo(e) => {
                write!(f, "io operation resulted in error: ({e:?})")
            }
            Error::InvalidPe(e) => {
                write!(f, "parsing .sys file resulted in error: ({e:?})")
            }
            Error::NonExistendPath(p) => {
                write!(f, "file '{}' does not exist", p.display())
            }
            Error::InvalidExtension(p) => {
                write!(f, "file '{}' is no .sys file", p.display())
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::BadIo(e) => Some(e),
            Self::InvalidPe(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::BadIo(value)
    }
}

impl From<peview::error::Error> for Error {
    fn from(value: peview::error::Error) -> Self {
        Error::InvalidPe(value)
    }
}
