mod environment;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
use crate::interpreter::Interpreter;
use crate::scanner::Scanner;
use parser::Parser;
use std::{
    env, fs,
    io::{stdin, stdout, BufRead, Write},
    process::exit,
};
fn run(interpreter: &mut Interpreter, source: &str) -> Result<(), String> {
    let scan = &mut Scanner::new(source);
    let tokens = scan.scan_tokens()?;
    let parser = Parser::new(tokens);
    let stmts = parser.parse()?;
    interpreter.interpret(&stmts)?;
    Ok(())
}
fn run_file(path: &str) {
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
fn run_prompt() -> Result<(), std::io::Error> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        match stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!("could not flush: {}", e);
                exit(74);
            }
        }
        let mut buffer = String::new();
        let stdin = stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    exit(74)
                }
                run(&mut interpreter, &buffer).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                exit(74);
            }
        }
    }
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("Usage: <filename>");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        // REPL
        run_prompt().unwrap();
    }
}
