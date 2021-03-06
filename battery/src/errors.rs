//! Errors handling
//!
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// Battery routines error.
///
/// Since all operations are basically I/O of some kind,
/// this is a thin wrapper around `::std::io::Error` with option
/// to store custom description for debugging purposes.
#[derive(Debug)]
pub struct Error {
    source: io::Error,
    description: Option<&'static str>,
}

impl Error {
    pub fn not_found(description: &'static str) -> Error {
        Error {
            source: io::Error::from(io::ErrorKind::NotFound),
            description: Some(description),
        }
    }

    pub fn invalid_data(description: &'static str) -> Error {
        Error {
            source: io::Error::from(io::ErrorKind::InvalidData),
            description: Some(description),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description {
            Some(desc) => write!(f, "{}", desc),
            None => self.source.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            source: e,
            description: None,
        }
    }
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
mod nix_impl {
    use std::io;

    use super::Error;

    impl From<nix::Error> for Error {
        fn from(e: nix::Error) -> Self {
            match e {
                nix::Error::Sys(errno) => Error {
                    source: io::Error::from_raw_os_error(errno as i32),
                    description: Some(errno.desc()),
                },
                nix::Error::InvalidPath => Error {
                    source: io::Error::new(io::ErrorKind::InvalidInput, e),
                    description: Some("Invalid path"),
                },
                nix::Error::InvalidUtf8 => Error {
                    source: io::Error::new(io::ErrorKind::InvalidData, e),
                    description: Some("Invalid UTF-8 string"),
                },
                nix::Error::UnsupportedOperation => Error {
                    source: io::Error::new(io::ErrorKind::Other, e),
                    description: Some("Unsupported operation"),
                },
            }
        }
    }
}
