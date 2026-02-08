use std::io::{self, Write};

use minilang::interpreter::Interpreter;
use minilang::lexer::Lexer;
use minilang::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        repl();
        return;
    }

    let source = match std::fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args[1], e);
            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.run(&program) {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }

    for line in &interpreter.output {
        println!("{}", line);
    }
}

fn repl() {
    println!("minilang REPL (Ctrl+Z to exit)");
    let stdin = io::stdin();
    let mut interpreter = Interpreter::new();
    let mut line = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        line.clear();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(trimmed);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let stmts = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                continue;
            }
        };

        let prev_len = interpreter.output.len();
        if let Err(e) = interpreter.run(&stmts) {
            eprintln!("Runtime error: {}", e);
            continue;
        }

        for line in &interpreter.output[prev_len..] {
            println!("{}", line);
        }
    }
}
