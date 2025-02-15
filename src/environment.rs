use std::collections;

use super::{
    value::Value,
    error::{Error, ErrorKind}
};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: collections::HashMap<String, Value>
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            values: collections::HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Result<&Value, Error> {
        let value = self.values.get(name);

        if value.is_none() && self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(name);
        }

        value.ok_or_else(|| {
                Error::new(
                    ErrorKind::RuntimeError {
                        message: format!("undefined variable {}", name)
                    }
                )
            })
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), Error> {
        match self.values.entry(name.clone()) {
            entry @ collections::hash_map::Entry::Occupied(_) => {
                entry.insert_entry(value);
            },
            collections::hash_map::Entry::Vacant(_) => {
                if let Some(enclosing) = self.enclosing.as_mut() {
                    return enclosing.assign(name, value);
                }

                return Err(
                    Error::new(
                        ErrorKind::RuntimeError {
                            message: format!("undefined variable {}", name)
                        }
                    )
                );
            }
        }

        Ok(())
    }

    pub fn enclosing(self) -> Option<Box<Environment>> {
        self.enclosing
    }
}