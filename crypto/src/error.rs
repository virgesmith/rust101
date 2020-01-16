use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
  InvalidBase58Digits(String),
  SearchStringTooLong(String)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::InvalidBase58Digits(ref s) => write!(f, "invalid search string: {}", s),
      Error::SearchStringTooLong(ref s) => write!(f, "search string is too long: {}", s),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

