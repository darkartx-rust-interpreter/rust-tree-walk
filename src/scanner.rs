use std::{iter, str};

use super::{
    token::{TokenType, Token},
    error::{Error, ErrorKind}
};

#[derive(Debug)]
pub struct Scanner {
    source: String
}

impl Scanner {
    pub fn from_str(source: &str) -> Self {
        Self {
            source: source.into()
        }
    }

    pub fn tokens(&self) -> ScannerIter {
        ScannerIter::new(self.source.chars())
    }

    pub fn source_ref(&self) -> &str {
        &self.source
    }
}

pub struct ScannerIter<'a> {
    source: str::Chars<'a>,
    buffer: Vec<char>,
    char_index: usize,
    line: usize
}

impl<'a> ScannerIter<'a> {
    fn new(source: str::Chars<'a>) -> Self {
        Self {
            source,
            buffer: Vec::new(),
            char_index: 0,
            line: 1
        }
    }

    fn buffer_char(&mut self, c: char) {
        self.buffer.push(c);
        self.char_index -= 1;
    }

    fn next_char(&mut self) -> Option<char> {
        self.char_index += 1;

        if let Some(c) = self.buffer.pop() {
            return Some(c);
        }

        self.source.next()
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let token = loop {
            let c = self.next_char();

            match c {
                None => break None,
                Some('(') => break Some(Token::new(TokenType::LeftParen, None, self.line)),
                Some(')') => break Some(Token::new(TokenType::RightParen, None, self.line)),
                Some('{') => break Some(Token::new(TokenType::LeftBrace, None, self.line)),
                Some('}') => break Some(Token::new(TokenType::RightBrace, None, self.line)),
                Some('.') => break Some(Token::new(TokenType::Dot, None, self.line)),
                Some('-') => break Some(Token::new(TokenType::Minus, None, self.line)),
                Some('+') => break Some(Token::new(TokenType::Plus, None, self.line)),
                Some(';') => break Some(Token::new(TokenType::Semicolon, None, self.line)),
                Some('*') => break Some(Token::new(TokenType::Star, None, self.line)),
                Some(',') => break Some(Token::new(TokenType::Comma, None, self.line)),
                Some('?') => break Some(Token::new(TokenType::Query, None, self.line)),
                Some(':') => break Some(Token::new(TokenType::Colon, None, self.line)),
                Some('!') => break Some(self.scan_op_equal(TokenType::Bang, TokenType::BangEqual)),
                Some('=') => break Some(self.scan_op_equal(TokenType::Equal, TokenType::EqualEqual)),
                Some('>') => break Some(self.scan_op_equal(TokenType::Greater, TokenType::GreaterEqual)),
                Some('<') => break Some(self.scan_op_equal(TokenType::Less, TokenType::LessEqual)),
                Some('/') => {
                    let token = self.scan_slash();
                    if token.is_some() { break token; }
                },
                Some('"') => break Some(self.scan_string()?),
                Some(c) if c.is_digit(10) => break Some(self.scan_number(c)),
                Some(c) if is_identifier_char(c)/* && !c.is_digit(10) */ => break Some(self.scan_identifier(c)),
                Some(c) if c.is_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                    }
                },
                Some(c) => {
                    return Err(
                        Error::new(
                            ErrorKind::ScannerError {
                                line: self.line,
                                message: format!("Unexpected character \"{c}\"")
                            }
                        )
                    );
                }
            };
        };

        Ok(token)
    }

    fn scan_op_equal(&mut self, op: TokenType, op_equal: TokenType) -> Token {
        let c = self.next_char();

        match c {
            Some('=') => Token::new(op_equal, None, self.line),
            _ => {
                if let Some(c) = c { self.buffer_char(c); }
                Token::new(op, None, self.line)
            }
        }
    }

    fn scan_slash(&mut self) -> Option<Token> {
        let c = self.next_char();

        match c {
            Some('/') => self.scan_single_line_comment(),
            Some('*') => self.scan_multi_line_comment(),
            _ => {
                if let Some(c) = c { self.buffer_char(c); }
                Some(Token::new(TokenType::Slash, None, self.line))
            }
        }
    }

    fn scan_single_line_comment(&mut self) -> Option<Token> {
        loop {
            match self.next_char() {
                Some('\n') => {
                    self.line += 1;
                    return None
                },
                None => { return None; },
                _ => {}
            }
        }
    }

    fn scan_multi_line_comment(&mut self) -> Option<Token> {
        loop {
            match self.next_char() {
                Some('\n') => { self.line += 1; },
                Some('*') => {
                    let c = self.next_char();

                    match c {
                        Some('\n') => { self.line += 1; },
                        Some('/') | None => return None,
                        _ => {}
                    }
                }
                Some('/') => {
                    let c = self.next_char();

                    match c {
                        Some('\n') => { self.line += 1; }
                        Some('*') => { self.scan_multi_line_comment(); },
                        None => return None,
                        _ => {}
                    }
                }
                None => { return None; },
                _ => {}
            }
        }
    }

    fn scan_string(&mut self) -> Result<Token, Error> {
        let mut value = String::new();

        loop {
            let c = self.next_char();

            match c {
                Some('\"') => break,
                Some(c) => {
                    if c == '\n' {
                        self.line += 1;
                    }

                    value.push(c);
                },
                None => {
                    return Err(
                        Error::new(
                            ErrorKind::ScannerError {
                                 line: self.line,
                                 message: "Unterminated string".into()
                            }
                        )
                    )
                }
            }
        }

        Ok(Token::new(TokenType::String, Some(value), self.line))
    }

    fn scan_number(&mut self, first: char) -> Token {
        let mut value: String = first.into();
        let mut have_dot = false;

        loop {
            let c = self.next_char();

            match c {
                Some('.') => {
                    if have_dot {
                        self.buffer_char('.');
                        break;
                    }

                    value.push('.');
                    have_dot = true;
                },
                Some(c ) if c.is_digit(10) => {
                    value.push(c);
                },
                c @ _ => {
                    if let Some(c) = c {
                        self.buffer_char(c);
                    }

                    break;
                }
            }
        }

        Token::new(TokenType::Number, Some(value), self.line)
    }

    fn scan_identifier(&mut self, c: char) -> Token {
        let mut value: String = c.into();

        loop {
            let c = self.next_char();

            match c {
                Some(c) if is_identifier_char(c) => {
                    value.push(c);
                }
                c @ _ => {
                    if let Some(c) = c {
                        self.buffer_char(c);
                    }

                    break;
                }
            }
        }

        let key_word = value.to_lowercase();

        match key_word.as_str() {
            "and"       => Token::new(TokenType::And, None, self.line),
            "class"     => Token::new(TokenType::Class, None, self.line),
            "else"      => Token::new(TokenType::Else, None, self.line),
            "false"     => Token::new(TokenType::False, None, self.line),
            "for"       => Token::new(TokenType::For, None, self.line),
            "fun"       => Token::new(TokenType::Fun, None, self.line),
            "if"        => Token::new(TokenType::If, None, self.line),
            "null"      => Token::new(TokenType::Null, None, self.line),
            "or"        => Token::new(TokenType::Or, None, self.line),
            "print"     => Token::new(TokenType::Print, None, self.line),
            "return"    => Token::new(TokenType::Return, None, self.line),
            "super"     => Token::new(TokenType::Super, None, self.line),
            "this"      => Token::new(TokenType::This, None, self.line),
            "true"      => Token::new(TokenType::True, None, self.line),
            "var"       => Token::new(TokenType::Var, None, self.line),
            "while"     => Token::new(TokenType::While, None, self.line),
            _           => Token::new(TokenType::Identifier, Some(value), self.line)
        }
    }
}

impl<'a> iter::Iterator for ScannerIter<'a> {
    type Item = Result<Token, Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(err) => Some(Err(err))
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

