use std::fmt;

use super::{
    token::Token,
    value::Value
};

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn Visitor);
}

#[derive(Debug)]
pub struct Binary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>
}

impl Binary {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right
        }
    }

    pub fn left(&self) -> &dyn Expression {
        self.left.as_ref()
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expression {
        self.right.as_ref()
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
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression
        }
    }

    pub fn expression(&self) -> &dyn Expression {
        self.expression.as_ref()
    }
}

impl Expression for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_grouping(self);
    }
}

#[derive(Debug)]
pub struct Literal {
    value: Value
}

impl Literal {
    pub fn new(value: Value) -> Self {
        Self {
            value
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
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
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Self {
            operator,
            right
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expression {
        self.right.as_ref()
    }
}

impl Expression for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self);
    }
}

#[derive(Debug)]
pub struct Ternary {
    operator: Token,
    first: Box<dyn Expression>,
    second: Box<dyn Expression>,
    third: Box<dyn Expression>
}

impl Ternary {
    pub fn new(
        operator: Token,
        first: Box<dyn Expression>,
        second: Box<dyn Expression>,
        third: Box<dyn Expression>
    ) -> Self {
        Self {
            operator,
            first,
            second,
            third
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn first(&self) -> &dyn Expression {
        self.first.as_ref()
    }

    pub fn second(&self) -> &dyn Expression {
        self.second.as_ref()
    }

    pub fn third(&self) -> &dyn Expression {
        self.third.as_ref()
    }
}

impl Expression for Ternary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_ternary(self);
    }
}

pub trait Visitor: fmt::Debug {
    fn visit_binary(&mut self, expression: &Binary);
    fn visit_grouping(&mut self, expression: &Grouping);
    fn visit_literal(&mut self, expression: &Literal);
    fn visit_unary(&mut self, expression: &Unary);
    fn visit_ternary(&mut self, expression: &Ternary);
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
        print!("{}", expression.value);
    }

    fn visit_unary(&mut self, expression: &Unary) {
        print!("({} ", expression.operator);
        expression.right.accept(self);
        print!(")")
    }
    
    fn visit_ternary(&mut self, expression: &Ternary) {
        print!("({} ", expression.operator);
        expression.first.accept(self);
        print!(" ");
        expression.second.accept(self);
        print!(" ");
        expression.third.accept(self);
        print!(")");
    }
}
