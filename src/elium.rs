fn main() {
    let code = r#"
    package test
    msg() "Hello World!"
    exit:
    "#;

    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    while let Ok(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => eprintln!("Error: {}", e),
    }
}
