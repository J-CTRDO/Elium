#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Package(String),
    Msg(String),
    Exit,
    Literal(Value),
    BinaryOp(Box<ASTNode>, String, Box<ASTNode>), // 追加: 二項演算子
    If(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>), // 追加: 条件分岐
    Variable(String),                                 // 追加: 変数
    Unexpected(String),                               // 想定外のトークン
}

#[derive(Debug)]
pub enum Value {
    Number(i64),
    Text(String),
    Boolean(bool),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::Package(name) => {
                    statements.push(ASTNode::Package(name.clone()));
                }
                Token::Msg(msg) => {
                    statements.push(ASTNode::Msg(msg.clone()));
                }
                Token::Exit => {
                    statements.push(ASTNode::Exit);
                }
                Token::Identifier(name) => {
                    if let Some(Token::LeftParen) = self.next_token() {
                        let args = self.parse_arguments()?;
                        statements.push(ASTNode::FunctionCall(name.clone(), args));
                    } else {
                        statements.push(ASTNode::Variable(name.clone()));
                    }
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
        }

        Ok(ASTNode::Program(statements))
    }

    fn parse_arguments(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut args = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                Token::RightParen => break,
                _ => {
                    // ここでトークンが変数かリテラルなのかを判定
                    match token {
                        Token::Text(s) => args.push(ASTNode::Literal(Value::Text(s))),
                        Token::Number(n) => args.push(ASTNode::Literal(Value::Number(*n))),
                        _ => return Err(format!("Unexpected argument token: {:?}", token)),
                    }
                }
            }
        }

        Ok(args)
    }
}
