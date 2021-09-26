use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    DateFormatError,
    FixedFieldLengthError,
}

use self::Error::*;

impl error::Error for Error {

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DateFormatError => write!(f, "date format error"),
            FixedFieldLengthError => write!(f, "fixed field length error"),
        }
    }
}
