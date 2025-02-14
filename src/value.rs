use std::fmt;

use super::{
    utils::parse_number,
    error::{Error, ErrorKind}
};

#[derive(Debug, Clone)]
pub enum Value {
    True,
    False,
    Null,
    String(String),
    Number(f64)
}

impl Value {
    pub fn as_number(&self) -> Result<Value, Error> {
        use Value::*;

        match self {
            True => Ok(Number(1.0_f64)),
            False | Null => Ok(Number(0.0_f64)),
            Number(number) => Ok(Number(*number)),
            String(str) => {
                match parse_number(str) {
                    Ok(value) => Ok(Number(value)),
                    Err(err) => return Err(
                        Error::new(
                            ErrorKind::RuntimeError { message: err.to_string() }
                        )
                    )
                }
            }
        }
    }

    pub fn as_boolean(&self) -> Value {
        use Value::*;

        match self {
            value @ (True | False) => value.clone(),
            Null => False,
            Number(number) => {
                if *number == 0.0 {
                    False
                } else {
                    True
                }
            },
            String(str) => {
                if str.is_empty() {
                    False
                } else {
                    True
                }
            }
        }
    }

    pub fn as_string(&self) -> Value {
        use Value::*;

        let value = match self {
            True => "true".into(),
            False => "false".into(),
            Null => "".into(),
            Number(number) => format!("{}", number),
            String(str) => str.clone()
        };

        Value::String(value)
    }

    pub fn is_null(&self) -> bool {
        if let Value::Null = self {
            true
        } else {
            false
        }
    }

    pub fn is_true(&self) -> bool {
        if let Value::True = self {
            true
        } else {
            false
        }
    }

    pub fn is_false(&self) -> bool {
        if let Value::False = self {
            true
        } else {
            false
        }
    }

    pub fn subtract(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => Ok(Number(lhs - rhs)),
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn division(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => Ok(Number(lhs / rhs)),
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn mutiply(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => Ok(Number(lhs * rhs)),
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn add(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self {
            String(lhs) => match rhs.as_string() {
                String(rhs) => Ok(String(format!("{}{}", lhs, rhs))),
                _ => unreachable!()
            },
            lhs @ _ => match lhs.as_number()? {
                Number(lhs) => match rhs.as_number()? {
                    Number(rhs) => Ok(Number(lhs + rhs)),
                    _ => unreachable!()
                },
                _ => unreachable!()
            }
        }
    }

    pub fn greater(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => {
                    if lhs > rhs {
                        Ok(True)
                    } else {
                        Ok(False)
                    }
                }
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn greater_equal(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => {
                    if lhs >= rhs {
                        Ok(True)
                    } else {
                        Ok(False)
                    }
                }
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn less(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => {
                    if lhs < rhs {
                        Ok(True)
                    } else {
                        Ok(False)
                    }
                }
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn less_equal(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.as_number()? {
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => {
                    if lhs <= rhs {
                        Ok(True)
                    } else {
                        Ok(False)
                    }
                }
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    pub fn equal(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self {
            True => if rhs.is_true() { Ok(True) } else { Ok(False) },
            False => if rhs.is_false() { Ok(True) } else { Ok(False) },
            Null => if rhs.is_null() { Ok(True) } else { Ok(False) },
            Number(lhs) => match rhs.as_number()? {
                Number(rhs) => if *lhs == rhs { Ok(True) } else { Ok(False) },
                _ => unreachable!()
            },
            String(lhs) => match rhs.as_string() {
                String(rhs) => if lhs.as_str() == rhs.as_str() { Ok(True) } else { Ok(False) },
                _ => unreachable!()
            }
        }
    }

    pub fn not_equal(&self, rhs: &Value) -> Result<Value, Error> {
        use Value::*;

        match self.equal(rhs)? {
            True => Ok(False),
            False => Ok(False),
            _ => unreachable!()
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;

        match self {
            True => write!(f, "true"),
            False => write!(f, "false"),
            Null => write!(f, "null"),
            String(value) => write!(f, "{}", value),
            Number(value) => write!(f, "{}", value)
        }
    }
}
