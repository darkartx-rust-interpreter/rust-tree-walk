use std::{fmt, error};

#[derive(Debug)]
pub(super) enum ErrorKind {
    ScannerError {
        line: usize,
        message: String
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub(super) fn new(kind: ErrorKind) -> Self {
        Self {
            kind
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match &self.kind {
            ScannerError { line, message } => {
                write!(f, "Error: {message} in {line}")
            }
        }
    }
}

impl error::Error for Error { }
