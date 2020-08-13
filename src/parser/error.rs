use std::fmt::{Debug, Display};

pub struct ParseError {
    error: String,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
impl From<String> for ParseError {
    fn from(error: String) -> Self {
        ParseError { error }
    }
}
impl From<&str> for ParseError {
    fn from(error: &str) -> Self {
        ParseError {
            error: error.to_string(),
        }
    }
}
