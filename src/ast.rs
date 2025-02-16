use std::fmt;

use super::{
    token::Token,
    value::Value
};

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor);

    fn as_variable(&self) -> Option<&Variable> {
        None
    }
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_ternary(self);
    }
}

#[derive(Debug)]
pub struct Variable {
    name: String
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Expression for Variable {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_variable(self);
    }

    fn as_variable(&self) -> Option<&Variable> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct Assign {
    name: String,
    value: Box<dyn Expression>
}

impl Assign {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        Self {
            name,
            value
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &dyn Expression {
        self.value.as_ref()
    }
}

impl Expression for Assign {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_assign(self);
    }
}

#[derive(Debug)]
pub struct Logical {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>
}

impl Logical {
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

impl Expression for Logical {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) {
        visitor.visit_logical(self);
    }
}

pub trait ExpressionVisitor: fmt::Debug {
    fn visit_binary(&mut self, expression: &Binary);
    fn visit_grouping(&mut self, expression: &Grouping);
    fn visit_literal(&mut self, expression: &Literal);
    fn visit_unary(&mut self, expression: &Unary);
    fn visit_ternary(&mut self, expression: &Ternary);
    fn visit_variable(&mut self, expression: &Variable);
    fn visit_assign(&mut self, expression: &Assign);
    fn visit_logical(&mut self, expression: &Logical);
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

impl ExpressionVisitor for Printer {
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
    
    fn visit_variable(&mut self, expression: &Variable) {
        print!("(variable \"{}\")", expression.name);
    }
    
    fn visit_assign(&mut self, expression: &Assign) {
        print!("(assign \"{}\" ", expression.name);
        expression.value.accept(self);
        print!(")");
    }
    
    fn visit_logical(&mut self, expression: &Logical) {
        print!("({} ", expression.operator);
        expression.left.accept(self);
        print!(" ");
        expression.right.accept(self);
        print!(")");
    }
}

pub trait Statement: fmt::Debug {
    fn accept(&self, visitor: &mut dyn StatementVisitor);
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression
        }
    }

    pub fn expression(&self) -> &dyn Expression {
        self.expression.as_ref()
    }
}

impl Statement for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_expression_statement(self);
    }
}

#[derive(Debug)]
pub struct Print {
    expression: Box<dyn Expression>,
}

impl Print {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression
        }
    }

    pub fn expression(&self) -> &dyn Expression {
        self.expression.as_ref()
    }
}

impl Statement for Print {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_print(self);
    }
}

#[derive(Debug)]
pub struct Var {
    name: String,
    right: Box<dyn Expression>
}

impl Var {
    pub fn new(name: String, right: Box<dyn Expression>) -> Self {
        Self {
            name,
            right
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn right(&self) -> &dyn Expression {
        self.right.as_ref()
    }
}

impl Statement for Var {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_var(self);
    }
}

#[derive(Debug)]
pub struct Block {
    statements: Vec<Box<dyn Statement>>,
}

impl Block {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self {
            statements
        }
    }

    pub fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_ref()
    }
}

impl Statement for Block {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_block(self);
    }
}

#[derive(Debug)]
pub struct If {
    condition: Box<dyn Expression>,
    then_branch: Box<dyn Statement>,
    else_branch: Option<Box<dyn Statement>>
}

impl If {
    pub fn new(condition: Box<dyn Expression>, then_branch: Box<dyn Statement>, else_branch: Option<Box<dyn Statement>>) -> Self {
        Self {
            condition,
            then_branch,
            else_branch
        }
    }

    pub fn condition(&self) -> &dyn Expression {
        self.condition.as_ref()
    }

    pub fn then_branch(&self) -> &dyn Statement {
        self.then_branch.as_ref()
    }

    pub fn else_branch(&self) -> Option<&dyn Statement> {
        self.else_branch.as_ref().map(Box::as_ref)
    }
}

impl Statement for If {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_if(self);
    }
}

#[derive(Debug)]
pub struct While {
    condition: Box<dyn Expression>,
    body: Box<dyn Statement>
}

impl While {
    pub fn new(condition: Box<dyn Expression>, body: Box<dyn Statement>) -> Self {
        Self {
            condition,
            body
        }
    }

    pub fn condition(&self) -> &dyn Expression {
        self.condition.as_ref()
    }

    pub fn body(&self) -> &dyn Statement {
        self.body.as_ref()
    }
}

impl Statement for While {
    fn accept(&self, visitor: &mut dyn StatementVisitor) {
        visitor.visit_while(self);
    }
}

pub trait StatementVisitor: fmt::Debug {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
    fn visit_print(&mut self, statement: &Print);
    fn visit_var(&mut self, statement: &Var);
    fn visit_block(&mut self, statement: &Block);
    fn visit_if(&mut self, statement: &If);
    fn visit_while(&mut self, statement: &While);
}
