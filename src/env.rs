//! The typing environment module and ok helper functions.

use std::fmt::Debug;

use crate::{
    err::{TypingError, TypingResult},
    expr::Expr,
};

#[derive( Clone, PartialEq)]
pub enum Type {
    /// The type of integers.
    Int,
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
    /// let mut env = Env { bindings: vec![] };
    ///
    /// let expr = Expr::Abs((("x".to_string(), Type::Int), Box::new(Expr::Term("x".to_string()))));
    /// assert_eq!(env.type_checking(&expr), Ok(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int))));
    /// ```
    pub fn type_checking(&mut self, expr: &Expr) -> TypingResult<Type> {
        match expr {
            Expr::Var(x) => {
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
                let lhs = match self.type_checking(e1) {
                    Ok(lhs) => lhs,
                    Err(e) => return Err(e),
                };

                self.add_binding(x.clone(), lhs.clone());
                let rhs = match self.type_checking(e2) {
                    Ok(rhs) => rhs,
                    Err(e) => return Err(e),
                };

                self.bindings.pop();
                Ok(rhs)
            }
            Expr::App((e1, e2)) => {
                let e1_type = match self.type_checking(e1) {
                    Ok(e1) => e1,
                    Err(e) => return Err(e),
                };
                let e2_type = match self.type_checking(e2) {
                    Ok(e2) => e2,
                    Err(e) => return Err(e),
                };

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
        }
    }
}
