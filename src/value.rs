use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    True,
    False,
    Null,
    String(String),
    Number(f64)
}

impl Value {
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
