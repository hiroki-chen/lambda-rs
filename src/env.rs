//! The typing environment module and ok helper functions.

use std::fmt::Debug;

use crate::{
    err::{TypingError, TypingResult},
    expr::{BinaryExpr, Expr, UnaryExpr},
};

#[derive(Clone, PartialEq)]
pub enum Type {
    /// The type of integers.
    Int,
    /// Booleans.
    Bool,
    /// The type of functions.
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone, Default)]
pub struct Env {
    bindings: Vec<(String, Type)>,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Arrow(arg, ret) => write!(f, "({:?} -> {:?})", arg, ret),
        }
    }
}

impl Env {
    pub fn add_binding(&mut self, x: String, ty: Type) {
        self.bindings.push((x, ty));
    }

    pub fn empty_env() -> Self {
        Self::default()
    }

    /// Statically checks the type of an expression.
    ///
    /// Returns `None` if the expression is ill-typed or `Ok(ty)` if the expression is well-typed.
    ///
    /// # Examples
    ///
    /// ```
    /// use stlc::env::{Env, Type};
    /// use stlc::expr::Expr;
    ///
    /// let mut env = Env::empty_env();
    ///
    /// let expr = Expr::Abs((("x".to_string(), Type::Int), Box::new(Expr::Term("x".to_string()))));
    /// assert_eq!(env.type_checking(&expr), Ok(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int))));
    /// ```
    pub fn type_checking(&mut self, expr: &Expr) -> TypingResult<Type> {
        match expr {
            Expr::Var(x) => {
                // Lexical scoping.
                for (y, ty) in self.bindings.iter().rev() {
                    if x == y {
                        return Ok(ty.clone());
                    }
                }
                Err(TypingError::UnboundVariable(x.clone()))
            }
            Expr::Term(_) => Ok(Type::Int),
            Expr::Abs(((x, ty), e)) => {
                self.add_binding(x.clone(), ty.clone());
                match self.type_checking(e) {
                    Ok(res_type) => {
                        self.bindings.pop();
                        Ok(Type::Arrow(Box::new(ty.clone()), Box::new(res_type)))
                    }
                    Err(e) => Err(e),
                }
            }
            Expr::Let((x, e1, e2)) => {
                let lhs = self.type_checking(e1)?;
                self.add_binding(x.clone(), lhs.clone());
                let rhs = self.type_checking(e2)?;

                self.bindings.pop();
                Ok(rhs)
            }
            Expr::IfElse((cond, conseq, alt)) => {
                let cond_type = self.type_checking(cond)?;

                if cond_type != Type::Bool {
                    return Err(TypingError::TypeMismatch(Type::Bool, cond_type));
                }

                let conseq_type = self.type_checking(conseq)?;
                let alt_type = self.type_checking(alt)?;

                if conseq_type != alt_type {
                    return Err(TypingError::TypeMismatch(conseq_type, alt_type));
                }

                Ok(conseq_type)
            }
            Expr::App((e1, e2)) => {
                let e1_type = self.type_checking(e1)?;
                let e2_type = self.type_checking(e2)?;

                match e1_type {
                    Type::Arrow(arg, ret) => {
                        if *arg == e2_type {
                            Ok(*ret.clone())
                        } else {
                            Err(TypingError::TypeMismatch(*arg, e2_type))
                        }
                    }
                    ty => Err(TypingError::TypeMismatch(
                        Type::Arrow(Box::new(e2_type), Box::new(Type::Int)),
                        ty,
                    )),
                }
            }

            Expr::Binary(expr) => {
                let (e1, e2) = expr.extract_operands();
                let e1_type = self.type_checking(&e1)?;
                let e2_type = self.type_checking(&e2)?;

                match expr {
                    BinaryExpr::Arith(_) => {
                        if e1_type == Type::Int && e2_type == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err(TypingError::TypeMismatch(Type::Int, e1_type))
                        }
                    }
                    BinaryExpr::Logical(_) => {
                        if e1_type == Type::Bool && e2_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(TypingError::TypeMismatch(Type::Bool, e1_type))
                        }
                    }
                }
            }
            Expr::Unary(expr) => {
                let e_type = self.type_checking(&expr.extract_operand())?;

                match expr {
                    UnaryExpr::Not(_) => match e_type {
                        Type::Int => Ok(Type::Int),
                        _ => Err(TypingError::TypeMismatch(Type::Int, e_type)),
                    },
                    UnaryExpr::Neg(_) => match e_type {
                        Type::Bool => Ok(Type::Bool),
                        _ => Err(TypingError::TypeMismatch(Type::Bool, e_type)),
                    },
                }
            }
        }
    }
}
