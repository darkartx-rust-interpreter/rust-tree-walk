/*
https://craftinginterpreters.com/parsing-expressions.html

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

use std::iter;

use super::{
    token::{Token, TokenType},
    ast::{Expression, Literal, Grouping, Unary, Binary},
    error::{Error, ErrorKind},
    value::Value
};

use TokenType::*;

type TokenResult = Result<Token, Error>;

pub struct Parser<'a> {
    tokens: Tokens<'a>
}

impl<'a> Parser<'a> {
    pub fn new<T: iter::Iterator<Item = TokenResult>>(tokens: &'a mut T) -> Self {
        let tokens = Tokens::new(tokens);

        Self {
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expression>, Error> {
        self.tokens.next()?;
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        self.equaity()
    }

    fn equaity(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.comparison()?;

        loop {
            let token = self.tokens.current();

            match token.as_ref().map(Token::token_type) {
                Some(BangEqual | EqualEqual) => {
                    let operator = token.unwrap();
                    self.tokens.next()?;
                    let right = self.comparison()?;
        
                    expression = Box::new(
                        Binary::new(
                            expression,
                            operator,
                            right
                        )
                    );
                },
                _ => break
            }
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.term()?;

        loop {
            let token = self.tokens.current();

            match token.as_ref().map(Token::token_type) {
                Some(Greater | GreaterEqual | Less | LessEqual) => {
                    let operator = token.unwrap();
                    self.tokens.next()?;
                    let right = self.term()?;

                    expression = Box::new(
                        Binary::new(
                            expression,
                            operator,
                            right
                        )
                    );
                },
                _ => break
            }
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.factor()?;

        loop {
            let token = self.tokens.current();

            match token.as_ref().map(Token::token_type) {
                Some(Minus | Plus) => {
                    let operator = token.unwrap();
                    self.tokens.next()?;
                    let right = self.factor()?;

                    expression = Box::new(
                        Binary::new(
                            expression,
                            operator,
                            right
                        )
                    );
                },
                _ => break
            }
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.unary()?;

        loop {
            let token = self.tokens.current();

            match token.as_ref().map(Token::token_type) {
                Some(Slash | Star) => {
                    let operator = token.unwrap();
                    self.tokens.next()?;
                    let right = self.unary()?;

                    expression = Box::new(
                        Binary::new(
                            expression,
                            operator,
                            right
                        )
                    );
                },
                _ => break
            }
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.tokens.current();

        match token.as_ref().map(Token::token_type) {
            Some(Bang | Minus) => {
                let operator = token.unwrap();
                self.tokens.next()?;
                let right = self.unary()?;

                Ok(Box::new(Unary::new(operator, right)))
            },
            _ => self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.tokens.current();

        self.tokens.next()?;

        match token.as_ref().map(Token::token_type) {
            Some(False) => Ok(Box::new(Literal::new(Value::False))),
            Some(True) => Ok(Box::new(Literal::new(Value::True))),
            Some(Null) => Ok(Box::new(Literal::new(Value::Null))),
            Some(Number | String) => Ok(Box::new(Literal::new(parse_value(token.unwrap())?))),
            Some(LeftParen) => {
                let expression = self.expression()?;
                let token = self.tokens.current();

                match token.as_ref().map(Token::token_type) {
                    Some(RightParen) => Ok(Box::new(Grouping::new(expression))),
                    _ => Err(
                        Error::new(
                            ErrorKind::ParserError {
                                token: Some(token.unwrap().clone()),
                                message: format!("Expect \")\" after expression")
                            }
                        )
                    )
                }
            },
            Some(_) => {
                let token = token.unwrap();

                Err(
                    Error::new(
                        ErrorKind::ParserError {
                            token: Some(token.clone()),
                            message: format!("Unexpeceted token \"{token}\"")
                        }
                    )
                )
            },
            None => Err(
                Error::new(
                    ErrorKind::ParserError {
                        token: None,
                        message: format!("Expect token")
                    }
                )
            )
        }
    }
}

struct Tokens<'a> {
    inner: &'a mut dyn iter::Iterator<Item = TokenResult>,
    current: Option<Token>
}

impl<'a> Tokens<'a> {
    fn new<T: iter::Iterator<Item = TokenResult>>(inner: &'a mut T) -> Self {
        Self {
            inner,
            current: None
        }
    }

    fn next(&mut self) -> Result<Option<Token>, Error> {
        let token = self.inner.next().transpose()?;
        self.current = token;
        Ok(self.current())
    }

    fn current(&mut self) -> Option<Token> {
        self.current.as_ref().map(|t| t.clone())
    }
}

fn parse_value(token: Token) -> Result<Value, Error> {
    match token.token_type() {
        String | Number => {},
        _ => return Err(
            Error::new(
                ErrorKind::ParserError {
                    token: Some(token.clone()),
                    message: format!("Token {} has no value", token.token_type())
                }
            )
        )
    }

    let value = match token.lexeme() {
        None => return Err(
            Error::new(
                ErrorKind::ParserError {
                    token: Some(token.clone()),
                    message: format!("Token {} without value", token)
                }
            )
        ),
        Some(value) => value
    };

    let value = match token.token_type() {
        Number => {
            match value.parse::<f64>() {
                Ok(value) => Value::Number(value),
                Err(err) => return Err(
                    Error::new(
                        ErrorKind::ParserError {
                            token: Some(token.clone()),
                            message: err.to_string()
                        }
                    )
                )
            }
        },
        String => {
            Value::String(value.into())
        },
        _ => panic!()
    };

    Ok(value)
}

