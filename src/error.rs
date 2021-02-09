use std::error;
use std::fmt;

// TODO
// figure out a way to add custom
// Brigade errors here.
#[derive(Debug)]
pub enum BrigadeError {
    Reqwest(reqwest::Error),
}

impl fmt::Display for BrigadeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrigadeError::Reqwest(err) => write!(f, "reqwest error: {}", err.to_string()),
        }
    }
}

impl error::Error for BrigadeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            BrigadeError::Reqwest(ref e) => Some(e),
        }
    }
}

impl From<reqwest::Error> for BrigadeError {
    fn from(err: reqwest::Error) -> BrigadeError {
        BrigadeError::Reqwest(err)
    }
}
