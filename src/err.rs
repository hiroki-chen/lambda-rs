use std::{error::Error, fmt, result::Result};

pub enum EvalError {
    UnboundVariable(String),
    TypeMismatch,
    FileNotFound(String),
    ParseError(String),
}

impl fmt::Debug for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UnboundVariable(x) => write!(f, "Unbound variable: {}", x),
            EvalError::TypeMismatch => write!(f, "Type mismatch"),
            EvalError::FileNotFound(x) => write!(f, "File not found: {}", x),
            EvalError::ParseError(x) => write!(f, "Parse error: {}", x),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EvalError {}

pub type EvalResult<T> = Result<T, EvalError>;
