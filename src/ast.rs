#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Package(String),
    Import(String, Option<String>),
    Msg(String),
    Variable(String, Box<Expr>), // Exprに変更
    BinaryOp(Box<Expr>, String, Box<Expr>), // Tokenを文字列で抽象化
    If(Box<Expr>, Vec<ASTNode>, Vec<ASTNode>), // 条件部はExprで記述
    Function(String, Vec<String>, Vec<ASTNode>),
    FunctionCall(String, Vec<Expr>), // 引数をExprのリストに変更
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Binary(Box<Expr>, String, Box<Expr>), // 演算子を文字列として抽象化
    Input(String), // 入力プロンプト
}
