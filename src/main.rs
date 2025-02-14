// main.rs
mod lexer;
mod parser;
mod ast;
mod interpreter;
mod scope;
mod utils;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use ast::{ASTNode, Expr};
use utils::error::{Error, Result};

fn main() {
    let code = r#"
    package test
    msg() "Hello World!"
    exit:
    "#;

    // トークナイザを初期化
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    // トークンをすべて取得
    while let Ok(token) = lexer.next_token() {
        tokens.push(token);
    }

    // パーサを初期化してASTに変換
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            // ASTをデバッグ出力
            println!("{:#?}", ast);
            
            // インタプリタを使ってASTを実行
            let mut interpreter = Interpreter::new();
            if let Err(err) = interpreter.interpret(ast) {
                eprintln!("Runtime error: {}", err);
            }
        },
        Err(e) => {
            // パースエラーを出力
            eprintln!("Parse error: {}", e);
        },
    }
}
