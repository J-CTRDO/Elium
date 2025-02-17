// src/ast.rs

use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Package(String),
    Import(String, Option<String>),
    Msg(String),
    // リテラルは式としても使うので、ASTNode::Literal と Expr::Literal のどちらかを使います。
    Literal(Value),
    // 変数代入：変数名 と 右辺の式（Box<Expr>）
    Variable(String, Box<Expr>),
    // 二項演算子（文としては使わず、式で扱うことを推奨）
    // BinaryOpは Expr::BinaryOp を使うのでここでは省略
    // 条件文：条件（ASTNode として）、then 部分、else 部分（どちらも Vec<ASTNode>）
    If(Box<ASTNode>, Vec<ASTNode>, Vec<ASTNode>),
    // 関数定義：関数名、引数リスト、関数本体（文のリスト）
    Function(String, Vec<String>, Vec<ASTNode>),
    // 関数呼び出し：関数名、引数リスト（各引数は Expr とする）
    FunctionCall(String, Vec<Expr>),
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    None,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Input(String),
    FunctionCall(String, Vec<Expr>),
}
