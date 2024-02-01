use moss::interpreter::Interpreter;
use moss::parser::Parser;
use moss::scanner::Scanner;
use std::{fs, process::exit};
#[allow(dead_code)]
pub fn run_file(path: &str) {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Ok(contents) => {
            run(&mut interpreter, &contents).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            exit(74);
        }
    }
}

pub fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), String> {
    let scan = &mut Scanner::new(source);
    let tokens = scan.scan_tokens()?;
    let parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(&stmts)?;
    Ok(())
}
