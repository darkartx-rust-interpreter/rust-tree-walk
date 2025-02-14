use std::collections;

use super::{
    value::Value,
    error::{Error, ErrorKind}
};

#[derive(Debug)]
pub struct Environment {
    values: collections::HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: collections::HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &String) -> Result<&Value, Error> {
        self.values.get(name).ok_or_else(|| {
            Error::new(
                ErrorKind::RuntimeError {
                    message: format!("undefined variable {}", name)
                }
            )
        })
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), Error> {
        self.get(&name)?;

        self.values.entry(name).insert_entry(value);

        Ok(())
    }
}