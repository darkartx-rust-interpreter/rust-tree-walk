use std::{fmt, error};

use super::token::Token;

#[derive(Debug)]
pub(super) enum ErrorKind {
    ScannerError {
        line: usize,
        message: String
    },
    ParserError {
        token: Option<Token>,
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
            },
            ParserError { token, message } => {
                match token {
                    Some(token) => write!(f, "Error: {message} at \'{}\' in {}", token, token.line()),
                    None => write!(f, "Error: {message}")
                }
            }
        }
    }
}

impl error::Error for Error { }
