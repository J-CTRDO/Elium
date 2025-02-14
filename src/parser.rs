pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
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
                    if let Some(Token::String(value)) = self.next_token() {
                        nodes.push(ASTNode::Msg(value.clone()));
                    } else {
                        return Err("Expected message string".to_string());
                    }
                }
                Token::Exit => {
                    self.next_token();
                    nodes.push(ASTNode::Exit);
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
        }
        Ok(ASTNode::Program(nodes))
    }
}
