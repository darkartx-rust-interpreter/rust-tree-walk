use super::{
    scanner::Scanner,
    error::{Error, ErrorKind},
    parser::Parser,
    value::Value,
    ast::{Visitor, Binary, Grouping, Expression, Literal, Unary, Ternary},
    token::TokenType
};

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<Value>,
    error: Option<Error>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            error: None
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), Error> {
        let scanner = Scanner::from_str(code);
        let mut tokens = scanner.tokens();
        let mut parser = Parser::new(&mut tokens);
        let expression = parser.parse()?;

        let value = self.evaluate_expression(expression.as_ref())?;

        println!("{}", value);

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

impl Visitor for Interpreter {
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
}