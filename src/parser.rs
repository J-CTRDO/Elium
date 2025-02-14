// parser.rs

use crate::ast::{ASTNode, Value};
use crate::lexer::Token;
use crate::utils::error::{Error, Result};

#[derive(Debug, Clone)]
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

    pub fn parse(&mut self) -> Result<ASTNode> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek_token() {
            match token {
                Token::Package => {
                    self.next_token();
                    // 次に Identifier が来ると仮定
                    if let Some(Token::Identifier(name)) = self.next_token() {
                        statements.push(ASTNode::Package(name.clone()));
                    } else {
                        return Err(Error::Syntax("Expected package name".into()));
                    }
                }
                Token::Msg => {
                    self.next_token();
                    if let Some(Token::Text(msg)) = self.next_token() {
                        statements.push(ASTNode::Msg(msg.clone()));
                    } else {
                        return Err(Error::Syntax("Expected message string".into()));
                    }
                }
                Token::Exit => {
                    self.next_token();
                    statements.push(ASTNode::Exit);
                }
                Token::Identifier(name) => {
                    // この例では、Identifier が関数呼び出しか変数かを判定
                    self.next_token();
                    if let Some(Token::LeftParen) = self.peek_token() {
                        self.next_token(); // '(' を消費
                        let args = self.parse_arguments()?;
                        statements.push(ASTNode::FunctionCall(name.clone(), args));
                    } else {
                        statements.push(ASTNode::Variable(name.clone()));
                    }
                }
                _ => return Err(Error::Syntax(format!("Unexpected token: {:?}", token))),
            }
        }

        Ok(ASTNode::Program(statements))
    }

    fn parse_arguments(&mut self) -> Result<Vec<ASTNode>> {
        let mut args = Vec::new();
        while let Some(token) = self.peek_token() {
            match token {
                Token::RightParen => {
                    self.next_token(); // 消費する
                    break;
                }
                _ => {
                    match self.next_token() {
                        Some(Token::Text(s)) => args.push(ASTNode::Literal(Value::Text(s.clone()))),
                        Some(Token::Number(n)) => args.push(ASTNode::Literal(Value::Number(*n))),
                        Some(tok) => return Err(Error::Syntax(format!("Unexpected argument token: {:?}", tok))),
                        None => break,
                    }
                    if let Some(Token::Comma) = self.peek_token() {
                        self.next_token(); // カンマを消費
                    }
                }
            }
        }
        Ok(args)
    }
}
