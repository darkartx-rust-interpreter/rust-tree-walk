use std::{
    io::{self, Write, BufRead},
    fs,
    error
};

use rust_tree_walk::Interpreter;

type Error = Box<dyn error::Error>;

fn args() -> clap::ArgMatches {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(clap::Arg::new("file"))
        .get_matches()
}

fn main() {
    let args = args();

    if let Some(path) = args.get_one::<String>("file") {
        run_file(path).unwrap();
        return;
    }
    
    run_prompt().unwrap();
}

fn run_file(path: &str) -> Result<(), Error> {
    let code = fs::read_to_string(path)?;
    Interpreter::new().run(&code)?;

    Ok(())
}

fn run_prompt() -> Result<(), Error> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        write!(stdout.lock(), "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        if let Err(err) = interpreter.run(&buffer) {
            eprintln!("{err}");
        }
        buffer.clear();
    }
}
