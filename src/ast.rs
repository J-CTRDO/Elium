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
    FunctionCall(String, Vec<(String, ASTNode)>),
    Exit,
}
