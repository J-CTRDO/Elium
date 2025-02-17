// src/main.rs

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod scope;
mod utils;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use utils::error::{Error, Result};

fn main() {
    let code = r#"
    package elium
    Import from elium to os
    msg "Hello World!"
    x = 1 + 2
    name = input("What is your name?")
    msg "Hello, {name}"
    if(x == 3) {
        msg "x is 3"
    } else {
        msg "x is not 3"
    }
    function (name=add, a, b) {
        i = a + b
        return i
    }
    result = add(5, 10)
    exit
    "#;

    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    while let Some(Ok(token)) = lexer.next_token() {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
            let mut interpreter = Interpreter::new();
            let stmts = match ast {
                ast::ASTNode::Program(s) => s,
                other => vec![other],
            };
            if let Err(err) = interpreter.interpret(stmts) {
                eprintln!("Runtime error: {}", err);
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
