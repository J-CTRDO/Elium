#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Package(String),
    Import(String, Option<String>),
    Msg(String),
    Variable(String, Box<ASTNode>),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>, Vec<ASTNode>),
    Function(String, Vec<String>, Vec<ASTNode>),
    FunctionCall(String, Vec<Expr>), // 関数呼び出しの追加
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
    Array(Vec<Value>), // 配列型を追加
    Map(HashMap<String, Value>), // 辞書型を追加
    None,
}

pub enum Expr {
    Literal(Value),
    Variable(String),
    Binary(Box<Expr>, String, Box<Expr>),
    Input(String),
    FunctionCall(String, Vec<Expr>), // 関数呼び出し
}
