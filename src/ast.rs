use std::{fmt, convert};

use super::token::Token;

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn Visitor);
}

impl<T: Expression + 'static> convert::From<T> for Box<dyn Expression> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[derive(Debug)]
pub struct Binary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>
}

impl Binary {
    pub fn new<L: Expression + 'static, R: Expression + 'static>(left: L, operator: Token, right: R) -> Self {
        let left = left.into();
        let right = right.into();

        Self {
            left,
            operator,
            right
        }
    }
}

impl Expression for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_binary(self);
    }
}

#[derive(Debug)]
pub struct Grouping {
    expression: Box<dyn Expression>
}

impl Grouping {
    pub fn new<E: Expression + 'static>(expression: E) -> Self {
        let expression = expression.into();

        Self {
            expression
        }
    }
}

impl Expression for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_grouping(self);
    }
}

#[derive(Debug)]
pub struct Literal {
    token: Token
}

impl Literal {
    pub fn new(token: Token) -> Self {
        Self {
            token
        }
    }
}

impl Expression for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_literal(self);
    }
}

#[derive(Debug)]
pub struct Unary {
    operator: Token,
    right: Box<dyn Expression>
}

impl Unary {
    pub fn new<R: Expression + 'static>(operator: Token, right: R) -> Self {
        let right = right.into();

        Self {
            operator,
            right
        }
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self);
    }
}

pub trait Visitor: fmt::Debug {
    fn visit_binary(&mut self, expression: &Binary);
    fn visit_grouping(&mut self, expression: &Grouping);
    fn visit_literal(&mut self, expression: &Literal);
    fn visit_unary(&mut self, expression: &Unary);
}

#[derive(Debug)]
pub struct Printer;

impl Printer {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&mut self, expression: &dyn Expression) {
        expression.accept(self);
    }
}

impl Visitor for Printer {
    fn visit_binary(&mut self, expression: &Binary) {
        print!("({} ", expression.operator);
        expression.left.accept(self);
        print!(" ");
        expression.right.accept(self);
        print!(")");
    }

    fn visit_grouping(&mut self, expression: &Grouping) {
        print!("(group ");
        expression.expression.accept(self);
        print!(")");
    }

    fn visit_literal(&mut self, expression: &Literal) {
        print!("{}", expression.token);
    }

    fn visit_unary(&mut self, expression: &Unary) {
        print!("({} ", expression.operator);
        expression.right.accept(self);
        print!(")")
    }
}
