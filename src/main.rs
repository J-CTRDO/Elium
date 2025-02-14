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
use utils::error::{Error, Result};

fn main() {
    let code = r#"
    package test
    msg "Hello World!"
    exit
    "#;

    // トークナイザを初期化
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    // トークンをすべて取得
    while let Some(Ok(token)) = lexer.next_token() {
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
            // パーサーが返すASTがProgramの場合、内部の文リストを取り出す
            let stmts = match ast {
                ast::ASTNode::Program(stmts) => stmts,
                other => vec![other],
            };
            if let Err(err) = interpreter.interpret(stmts) {
                eprintln!("Runtime error: {}", err);
            }
        },
        Err(e) => {
            eprintln!("Parse error: {}", e);
        },
    }
}
