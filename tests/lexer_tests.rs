#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("set x = 42 msg x");
        assert_eq!(lexer.next_token(), Some(Token::Set));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Equals));
        assert_eq!(lexer.next_token(), Some(Token::Number(42)));
        assert_eq!(lexer.next_token(), Some(Token::Print));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_lexer_with_new_tokens() {
        let mut lexer = Lexer::new("if x * 10 else msg");
        assert_eq!(lexer.next_token(), Some(Token::If));
        assert_eq!(lexer.next_token(), Some(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Some(Token::Multiply));
        assert_eq!(lexer.next_token(), Some(Token::Number(10)));
        assert_eq!(lexer.next_token(), Some(Token::Else));
        assert_eq!(lexer.next_token(), Some(Token::Print));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_error_handling() {
        let mut lexer = Lexer::new("if x @ 10");
        assert_eq!(lexer.next_token(), Ok(Token::If));
        assert_eq!(lexer.next_token(), Ok(Token::Identifier("x".to_string())));

        // エラー発生箇所の確認
        let error = lexer.next_token().unwrap_err();
        assert_eq!(error.message, "Unexpected character: '@'");
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 6);
    }

    #[test]
    fn test_multiline_handling() {
        let mut lexer = Lexer::new("msg hello\nset x = 42");
        assert_eq!(lexer.next_token(), Ok(Token::Print));
        assert_eq!(lexer.next_token(), Ok(Token::Identifier("hello".to_string())));
        assert_eq!(lexer.next_token(), Ok(Token::Set));
        assert_eq!(lexer.next_token(), Ok(Token::Identifier("x".to_string())));
        assert_eq!(lexer.next_token(), Ok(Token::Equals));
        assert_eq!(lexer.next_token(), Ok(Token::Number(42)));
    }
    #[test]
fn test_boolean_expression() {
    let mut interpreter = Interpreter::new();
    let stmts = vec![
        Stmt::VariableDeclaration("x".to_string(), Expr::Literal(Value::Boolean(true))),
        Stmt::If(
            Expr::Variable("x".to_string()),
            vec![Stmt::Print(Expr::Literal(Value::Text("Condition met".to_string())))],
            Some(vec![Stmt::Print(Expr::Literal(Value::Text("Condition not met".to_string())))]),
        ),
    ];
    interpreter.interpret(stmts).unwrap();
}

}
