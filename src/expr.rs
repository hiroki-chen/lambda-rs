//! The module for expressions for the STLC.

use std::fmt::Debug;

use crate::env::Type;

#[derive(Clone, PartialEq)]
pub enum Expr {
    /// Integer literal: `1`, `2`, `3`, etc.
    Term(i32),
    /// Variable: `x`, `y`, `z`, etc.
    Var(String),
    /// Application: `e1 e2`.
    App((Box<Expr>, Box<Expr>)),
    /// Lambda abstraction: `λx.e`.
    Abs(((String, Type), Box<Expr>)),
    /// Equivalent to `let x = e1 in e2`.
    Let((String, Box<Expr>, Box<Expr>)),
    /// Equivalent to `if e1 then e2 else e3`.
    IfElse((Box<Expr>, Box<Expr>, Box<Expr>)),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Abs(((x, ty), e)) => write!(f, "λ{}:{:?}.{:?}", x, ty, e),
            Expr::App((e1, e2)) => write!(f, "({:?}) {:?}", e1, e2),
            Expr::Term(n) => write!(f, "{}", n),
            Expr::Var(x) => write!(f, "{}", x),
            Expr::IfElse((e1, e2, e3)) => write!(f, "if {:?} then {:?} else {:?}", e1, e2, e3),
            Expr::Let((x, e1, e2)) => write!(f, "let {} = {:?} in {:?}", x, e1, e2),
        }
    }
}
