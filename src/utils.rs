// utils.rs

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

pub mod scope {
    // 追加のスコープユーティリティが必要な場合、ここに記述
}

pub mod value {
    use crate::ast::Value;

    impl Value {
        pub fn as_number(&self) -> Option<i64> {
            if let Value::Number(n) = self {
                Some(*n)
            } else {
                None
            }
        }

        pub fn as_text(&self) -> Option<String> {
            if let Value::Text(s) = self {
                Some(s.clone())
            } else {
                None
            }
        }

        pub fn as_boolean(&self) -> Option<bool> {
            if let Value::Boolean(b) = self {
                Some(*b)
            } else {
                None
            }
        }
    }
}
