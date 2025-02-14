use crate::value;

use super::{
    scanner::Scanner,
    error::{Error, ErrorKind},
    parser::Parser,
    value::Value,
    ast::{
        ExpressionVisitor,
        StatementVisitor,
        Binary,
        Grouping,
        Expression,
        Literal,
        Unary,
        Ternary,
        ExpressionStatement,
        Print,
        Statement,
        Variable,
        Var,
        Assign
    },
    token::TokenType,
    environment::Environment
};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    stack: Vec<Value>,
    error: Option<Error>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            stack: Vec::new(),
            error: None
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), Error> {
        let scanner = Scanner::from_str(code);
        let mut tokens = scanner.tokens();
        let mut parser = Parser::new(&mut tokens);
        let statements = parser.parse()?;

        for statement in statements {
            self.evaluate_statement(statement.as_ref())?;
        }

        Ok(())
    }

    fn pop_from_stack(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or_else(|| {
            Error::new(
                ErrorKind::RuntimeError { message: "Expect value being in the stack".into() }
            )
        })
    }

    fn push_to_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn evaluate_statement(&mut self, statement: &dyn Statement) -> Result<(), Error> {
        statement.accept(self);
        self.handle_error()?;

        Ok(())
    }

    fn evaluate_expression(&mut self, expression: &dyn Expression) -> Result<Value, Error> {
        expression.accept(self);
        self.handle_error()?;

        self.pop_from_stack()
    }

    fn evaluate_binary(&mut self, expression: &Binary) -> Result<Value, Error> {
        let left = self.evaluate_expression(expression.left())?;
        let right = self.evaluate_expression(expression.right())?;
        let operator = expression.operator();

        use TokenType::*;

        let value = match operator.token_type() {
            Minus => left.subtract(&right)?,
            Slash => left.division(&right)?,
            Star => left.mutiply(&right)?,
            Plus => left.add(&right)?,
            Greater => left.greater(&right)?,
            GreaterEqual => left.greater_equal(&right)?,
            Less => left.less(&right)?,
            LessEqual => left.less_equal(&right)?,
            EqualEqual => left.equal(&right)?,
            BangEqual => left.not_equal(&right)?,
            _ => unreachable!()
        };

        Ok(value)
    }

    fn evaluate_unary(&mut self, expression: &Unary) -> Result<Value, Error> {
        let right = self.evaluate_expression(expression)?;
        let operator = expression.operator();

        use TokenType::*;

        let value = match operator.token_type() {
            Minus => {
                match right.as_number()? {
                    Value::Number(number) => Value::Number(-number),
                    _ => unreachable!()
                }
            },
            Bang => {
                match right.as_boolean() {
                    Value::True => Value::False,
                    Value::False => Value::True,
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };

        Ok(value)
    }

    fn evaluate_ternary(&mut self, expression: &Ternary) -> Result<Value, Error> {
        let operator = expression.operator();

        use TokenType::*;

        let value = match operator.token_type() {
            Query => {
                let condition = self.evaluate_expression(expression.first())?;

                if condition.as_boolean().is_true() {
                    self.evaluate_expression(expression.second())?
                } else {
                    self.evaluate_expression(expression.third())?
                }
            },
            _ => unreachable!()
        };

        Ok(value)
    }

    fn handle_error(&mut self) -> Result<(), Error> {
        if let Some(err) = self.error.take() {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_binary(&mut self, expression: &Binary) {
        let result = self.evaluate_binary(expression);

        match result {
            Ok(value) => self.push_to_stack(value),
            Err(error) => {
                self.error = Some(error)
            }
        }
    }

    fn visit_grouping(&mut self, expression: &Grouping) {
        let result = self.evaluate_expression(expression);

        match result {
            Ok(value) => self.push_to_stack(value),
            Err(error) => {
                self.error = Some(error)
            }
        }
    }

    fn visit_literal(&mut self, expression: &Literal) {
        self.push_to_stack(expression.value().clone());
    }

    fn visit_unary(&mut self, expression: &Unary) {
        let result = self.evaluate_unary(expression);

        match result {
            Ok(value) => self.push_to_stack(value),
            Err(error) => {
                self.error = Some(error)
            }
        }
    }

    fn visit_ternary(&mut self, expression: &Ternary) {
        let result = self.evaluate_ternary(expression);

        match result {
            Ok(value) => self.push_to_stack(value),
            Err(error) => {
                self.error = Some(error)
            }
        }
    }
    
    fn visit_variable(&mut self, expression: &Variable) {
        let result = self.environment.get(expression.name());

        match result {
            Ok(value) => {
                let value = value.clone();
                self.push_to_stack(value)
            },
            Err(error) => {
                self.error = Some(error)
            }
        }
    }
    
    fn visit_assign(&mut self, expression: &Assign) {
        let result = self.evaluate_expression(expression.value());

        let value = match result {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error);
                return;
            }
        };

        let result = self.environment.assign(expression.name().clone(), value);

        if let Err(error) = result {
            self.error = Some(error)
        }
    }
}

impl StatementVisitor for Interpreter {
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) {
        let result = self.evaluate_expression(statement.expression());

        match result {
            Ok(_value) => {},
            Err(error) => {
                self.error = Some(error)
            }
        }
    }

    fn visit_print(&mut self, statement: &Print) {
        let result = self.evaluate_expression(statement.expression());

        match result {
            Ok(value) => println!("{}", value),
            Err(error) => {
                self.error = Some(error)
            }
        }
    }
    
    fn visit_var(&mut self, statement: &Var) {
        let result = self.evaluate_expression(statement.right());

        match result {
            Ok(value) => {
                self.environment.define(statement.name().clone(), value);
            },
            Err(error) => {
                self.error = Some(error)
            }
        }
    }
}
