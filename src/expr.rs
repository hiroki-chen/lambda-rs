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
    /// Binary expression: `e1 + e2`, `e1 - e2`, etc.
    Binary(BinaryExpr),
    /// Unary expression
    Unary(UnaryExpr),
}

#[derive(Clone, PartialEq)]
pub enum UnaryExpr {
    Not(Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Clone, PartialEq)]
pub enum BinaryExpr {
    Logical(BinaryLogicalExpr),
    Arith(BinaryArithmeticExpr),
}

#[derive(Clone, PartialEq)]
pub enum BinaryLogicalExpr {
    /// Addition: `e1 + e2`.
    Add((Box<Expr>, Box<Expr>)),
    /// Subtraction: `e1 - e2`.
    Sub((Box<Expr>, Box<Expr>)),
    /// Multiplication: `e1 * e2`.
    Mul((Box<Expr>, Box<Expr>)),
    /// Division: `e1 / e2`.
    Div((Box<Expr>, Box<Expr>)),
    /// Modulo: `e1 % e2`.
    Mod((Box<Expr>, Box<Expr>)),
}

#[derive(Clone, PartialEq)]
pub enum BinaryArithmeticExpr {
    /// Less than: `e1 < e2`.
    Lt((Box<Expr>, Box<Expr>)),
    /// Less than or equal to: `e1 <= e2`.
    Le((Box<Expr>, Box<Expr>)),
    /// Greater than: `e1 > e2`.
    Gt((Box<Expr>, Box<Expr>)),
    /// Greater than or equal to: `e1 >= e2`.
    Ge((Box<Expr>, Box<Expr>)),
    /// Equality: `e1 == e2`.
    Eq((Box<Expr>, Box<Expr>)),
    /// Inequality: `e1 != e2`.
    Ne((Box<Expr>, Box<Expr>)),
}

impl Debug for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryExpr::Logical(e) => write!(f, "{:?}", e),
            BinaryExpr::Arith(e) => write!(f, "{:?}", e),
        }
    }
}

impl Debug for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryExpr::Not(e) => write!(f, "!{:?}", e),
            UnaryExpr::Neg(e) => write!(f, "-{:?}", e),
        }
    }
}

impl Debug for BinaryArithmeticExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryArithmeticExpr::Lt((e1, e2)) => write!(f, "{:?} < {:?}", e1, e2),
            BinaryArithmeticExpr::Le((e1, e2)) => write!(f, "{:?} <= {:?}", e1, e2),
            BinaryArithmeticExpr::Gt((e1, e2)) => write!(f, "{:?} > {:?}", e1, e2),
            BinaryArithmeticExpr::Ge((e1, e2)) => write!(f, "{:?} >= {:?}", e1, e2),
            BinaryArithmeticExpr::Eq((e1, e2)) => write!(f, "{:?} == {:?}", e1, e2),
            BinaryArithmeticExpr::Ne((e1, e2)) => write!(f, "{:?} != {:?}", e1, e2),
        }
    }
}

impl Debug for BinaryLogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryLogicalExpr::Add((e1, e2)) => write!(f, "{:?} + {:?}", e1, e2),
            BinaryLogicalExpr::Sub((e1, e2)) => write!(f, "{:?} - {:?}", e1, e2),
            BinaryLogicalExpr::Mul((e1, e2)) => write!(f, "{:?} * {:?}", e1, e2),
            BinaryLogicalExpr::Div((e1, e2)) => write!(f, "{:?} / {:?}", e1, e2),
            BinaryLogicalExpr::Mod((e1, e2)) => write!(f, "{:?} % {:?}", e1, e2),
        }
    }
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
            Expr::Binary(e) => write!(f, "{:?}", e),
            Expr::Unary(e) => write!(f, "{:?}", e),
        }
    }
}

impl UnaryExpr {
    pub fn extract_operand(&self) -> Box<Expr> {
        match self {
            UnaryExpr::Not(e) => e.clone(),
            UnaryExpr::Neg(e) => e.clone(),
        }
    }
}

impl BinaryExpr {
    pub fn extract_operands(&self) -> (Box<Expr>, Box<Expr>) {
        match self {
            BinaryExpr::Logical(e) => match e {
                BinaryLogicalExpr::Add((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Sub((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Mul((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Div((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Mod((e1, e2)) => (e1.clone(), e2.clone()),
            },
            BinaryExpr::Arith(e) => match e {
                BinaryArithmeticExpr::Lt((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryArithmeticExpr::Le((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryArithmeticExpr::Gt((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryArithmeticExpr::Ge((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryArithmeticExpr::Eq((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryArithmeticExpr::Ne((e1, e2)) => (e1.clone(), e2.clone()),
            },
        }
    }
}
