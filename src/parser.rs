/*
https://craftinginterpreters.com/parsing-expressions.html

expression     → equality ( ( "," ) equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
ternary        → unary ( ( "?" expression ":" expression ) )? ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

use std::iter;

use crate::token::TokenOption;

use super::{
    token::{Token, TokenType},
    ast::{
        Expression,
        Literal,
        Grouping,
        Unary,
        Binary,
        Ternary,
        Statement,
        Print,
        ExpressionStatement,
        Var,
        Variable,
        Assign
    },
    error::{Error, ErrorKind},
    value::Value,
    utils::parse_number
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

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Statement>>, Error> {
        self.tokens.next()?;
        let mut result = Vec::new();

        loop {
            let token = self.tokens.current();

            if token.is_some() {
                result.push(self.declaration()?);
            } else {
                break;
            }
        }

        Ok(result)
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        if self.tokens.token_match(&[Var]) {
            self.tokens.next()?;
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.tokens.consume(&[Identifier], "Expect variable name")?;

        let name = token.lexeme().unwrap().into();
        let initializer = if let Some(Equal) = self.tokens.current().token_type() {
            self.tokens.next()?;
            self.expression()?
        } else {
            Box::new(Literal::new(Value::Null))
        };

        self.tokens.consume(&[Semicolon], "Expect \";\" after expression")?;


        Ok(Box::new(Var::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        if self.tokens.token_match(&[Print]) {
            self.tokens.next()?;
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let expression = self.expression()?;
        self.tokens.consume(&[Semicolon], "Expect \";\" after value")?;

        Ok(Box::new(Print::new(expression)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let expression = self.expression()?;
        self.tokens.consume(&[Semicolon], "Expect \";\" after expression")?;

        Ok(Box::new(ExpressionStatement::new(expression)))
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.assignment()?;

        while self.tokens.token_match(&[Comma]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.assignment()?;
        
            expression = Box::new(
                Binary::new(
                    expression,
                    operator,
                    right
                )
            );
        }

        Ok(expression)
    }

    fn assignment(&mut self) -> Result<Box<dyn Expression>, Error> {
        let expression = self.equaity()?;

        if self.tokens.token_match(&[Equal]) {
            let token = self.tokens.next()?;

            match expression.as_variable() {
                Some(variable) => {
                    let name = variable.name().clone();
                    let value = self.assignment()?;

                    return Ok(Box::new(Assign::new(name, value)))
                },
                None => {
                    return Err(
                        Error::new(
                            ErrorKind::ParserError {
                                token,
                                message: "Invalid assignment target".into()
                            }
                        )
                    )
                }
            }
        }

        Ok(expression)
    }

    fn equaity(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.comparison()?;

        while self.tokens.token_match(&[BangEqual, EqualEqual]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.comparison()?;

            expression = Box::new(
                Binary::new(
                    expression,
                    operator,
                    right
                )
            );
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.term()?;

        while self.tokens.token_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.term()?;

            expression = Box::new(
                Binary::new(
                    expression,
                    operator,
                    right
                )
            );
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.factor()?;

        while self.tokens.token_match(&[Minus, Plus]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.factor()?;

            expression = Box::new(
                Binary::new(
                    expression,
                    operator,
                    right
                )
            );
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.ternary()?;

        while self.tokens.token_match(&[Slash, Star]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.ternary()?;

            expression = Box::new(
                Binary::new(
                    expression,
                    operator,
                    right
                )
            );
        }

        Ok(expression)
    }

    fn ternary(&mut self) -> Result<Box<dyn Expression>, Error> {
        let mut expression = self.unary()?;

        if self.tokens.token_match(&[Query]) {
            let operator = self.tokens.next()?.unwrap();
            let second = self.expression()?;
            self.tokens.consume(&[Colon], "Expected \":\" after first expression")?;
            let third = self.expression()?;

            expression = Box::new(
                Ternary::new(
                    operator,
                    expression,
                    second,
                    third
                )
            );
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, Error> {
        if self.tokens.token_match(&[Bang, Minus]) {
            let operator = self.tokens.next()?.unwrap();
            let right = self.unary()?;

            Ok(Box::new(Unary::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.tokens.next()?;

        match token.token_type() {
            Some(False) => Ok(Box::new(Literal::new(Value::False))),
            Some(True) => Ok(Box::new(Literal::new(Value::True))),
            Some(Null) => Ok(Box::new(Literal::new(Value::Null))),
            Some(Number | String) => Ok(Box::new(Literal::new(parse_value(token.unwrap())?))),
            Some(Identifier) => Ok(Box::new(Variable::new(token.lexeme().unwrap().into()))),
            Some(LeftParen) => {
                let expression = self.expression()?;
                self.tokens.consume(&[RightParen], "Expect \")\" after expression")?;

                Ok(Box::new(Grouping::new(expression)))
            },
            Some(_) => {
                let token = token.unwrap();

                Err(
                    Error::new(
                        ErrorKind::ParserError {
                            message: format!("Unexpeceted token \"{token}\""),
                            token: Some(token)
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
        let current = self.current.take();
        let token = self.inner.next().transpose()?;
        self.current = token;
        Ok(current)
    }

    fn current(&mut self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn consume(&mut self, variants: &[TokenType], err_message: &str) -> Result<Token, Error> {
        let token = self.next()?;

        let token = if let Some(token) = token { token } else {
            return Err(
                Error::new(
                    ErrorKind::ParserError {
                        token,
                        message: err_message.into()
                    }
                )
            );
        };

        let matched = variants.iter().any(|v| *v == token.token_type());

        if matched {
            Ok(token)
        } else {
            Err(
                Error::new(
                    ErrorKind::ParserError {
                        token: Some(token),
                        message: err_message.into()
                    }
                )
            )
        }
    }

    fn token_match(&self, variants: &[TokenType]) -> bool {
        let token = self.current.as_ref();

        if token.is_none() {
            return false;
        }

        let token = token.unwrap();

        variants.iter().any(|v| *v == token.token_type())
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
            match parse_number(value) {
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

