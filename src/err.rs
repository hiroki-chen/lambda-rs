use std::{
    error::Error,
    fmt::{Debug, Display},
    result::Result,
};

use crate::env::Type;

pub enum TypingError {
    UnboundVariable(String),
    TypeMismatch(Type, Type),
}

impl Debug for TypingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypingError::UnboundVariable(x) => write!(f, "Unbound variable: {}", x),
            TypingError::TypeMismatch(ty1, ty2) => {
                write!(f, "Expected type {:?}, got {:?}", ty1, ty2)
            }
        }
    }
}

impl Display for TypingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TypingError {}

pub type TypingResult<T> = Result<T, TypingError>;
