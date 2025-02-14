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

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut nodes = Vec::new();

        while let Some(token) = self.peek_token() {
            match token {
                Token::Package => {
                    self.next_token();
                    if let Some(Token::Identifier(name)) = self.next_token() {
                        nodes.push(ASTNode::Package(name.clone()));
                    } else {
                        return Err("Expected package name".to_string());
                    }
                }
                Token::Msg => {
                    self.next_token();
                    if let Some(Token::Text(value)) = self.next_token() {
                        nodes.push(ASTNode::Msg(value.clone()));
                    } else {
                        return Err("Expected message string".to_string());
                    }
                }
                Token::Exit => {
                    self.next_token();
                    nodes.push(ASTNode::Exit);
                }
                Token::Number(_) | Token::Text(_) => {
                    nodes.push(self.parse_expression()?);
                }
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
                }
            }
        }

        Ok(ASTNode::Program(nodes))
    }

    /// 式を解析
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let left = self.parse_primary()?;
        if let Some(Token::Operator(op)) = self.peek_token() {
            self.next_token();
            let right = self.parse_expression()?;
            Ok(ASTNode::BinaryOp(Box::new(left), op.clone(), Box::new(right)))
        } else {
            Ok(left)
        }
    }

    /// 基本的な値を解析
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        match self.next_token() {
            Some(Token::Number(n)) => Ok(ASTNode::Literal(Value::Number(*n))),
            Some(Token::Text(s)) => Ok(ASTNode::Literal(Value::Text(s.clone()))),
            Some(Token::Identifier(ident)) => Ok(ASTNode::Variable(ident.clone())),
            Some(token) => Err(format!("Unexpected token in primary: {:?}", token)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
    pub fn parse_function_call(&mut self) -> Result<ASTNode, String> {
        if let Some(Token::Identifier(name)) = self.next_token() {
            let mut args = Vec::new();
            if let Some(Token::LeftParen) = self.next_token() {
                while let Some(token) = self.peek_token() {
                    if let Token::RightParen = token {
                        self.next_token();
                        break;
                    } else {
                        args.push(self.parse_expression()?);
                        if let Some(Token::Comma) = self.peek_token() {
                            self.next_token(); // カンマをスキップ
                        }
                    }
                }
                Ok(ASTNode::FunctionCall(name.clone(), args))
            } else {
                Err("Expected '(' after function name".to_string())
            }
        } else {
            Err("Expected function name".to_string())
        }
    }    
}
