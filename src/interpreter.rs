use super::{
    scanner::Scanner,
    error::Error
};

#[derive(Debug)]
pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run(&mut self, code: &str) -> Result<(), Error> {
        let scanner = Scanner::from_str(code);

        for token in scanner.tokens() {
            let token = token?;
            println!("{token:?}");
        }

        Ok(())
    }
}