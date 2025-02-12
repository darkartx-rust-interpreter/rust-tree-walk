use super::{
    scanner::Scanner,
    error::Error,
    parser::Parser,
    ast::Printer
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
        let mut tokens = scanner.tokens();
        let mut parser = Parser::new(&mut tokens);
        let ast = parser.parse()?;

        let mut printer = Printer::new();
        ast.accept(&mut printer);
        println!();

        Ok(())
    }
}