// src/utils.rs

pub mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        Runtime(String),
        Syntax(String),
        Type(String),
        UnexpectedEOF,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Runtime(msg) => write!(f, "Runtime Error: {}", msg),
                Error::Syntax(msg) => write!(f, "Syntax Error: {}", msg),
                Error::Type(msg) => write!(f, "Type Error: {}", msg),
                Error::UnexpectedEOF => write!(f, "Unexpected end of input"),
            }
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}
