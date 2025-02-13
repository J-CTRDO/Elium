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
}
