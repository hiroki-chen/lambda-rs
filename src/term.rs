//! The module for expressions for the STLC.

use std::fmt;

use crate::{clos::Closure, env::EvalCtx};

pub type Type = Value;

#[derive(Clone, Debug, PartialEq)]
pub enum VariableName {
    Global(String),
    Local(usize),
    Quote(usize),
}

/// This represents the term in our core lambda calculus.
///
/// Note that since our type system is dependently typed, there is no longer a syntactic
/// distinction between these things because types now include terms that can be reduced,
/// computed, and evaluated.
///
/// Consider for example now we have a vector type:
///
/// ```haskell
/// {-# LANGUAGE GADTs #-}
/// data Vec (n :: Nat) (a :: Type) where
///     VNil  :: Vec Z a
///     VCons :: a -> Vec n a -> Vec (S n) a
/// ```
///
/// Now suppose we have `a: Vec (1 + 2 + 3) Nat` and `b: Vec 6 Nat`, to derive the type
/// `a ≡ b` (requires `eq_rect`), we must allow computation to occur inside types.
#[derive(Clone, PartialEq)]
pub enum Term {
    /// x: ρ
    AnnotatedTerm {
        term: Box<CheckableTerm>,
        ty: Box<CheckableTerm>,
    },
    /// Integer literal: `1`, `2`, `3`, etc.
    Lit(LitTerm),
    /// Variable: `x`, `y`, `z`, etc. used to look up the evaluation environment.
    Var(VariableName),
    /// Bounded
    Bounded(usize),
    /// Application: `e1 e2`.
    App {
        clos: Box<Term>,
        arg: Box<CheckableTerm>,
    },
    /// For example, polymorphism functions like `∀x:*. x -> x`
    /// or `∀(A: *). A -> A` must be declared this way.
    DependentFunctionSpace {
        arg: Box<CheckableTerm>,
        ret: Box<CheckableTerm>,
    },
    // TODO: Determine the level of type universe?
    // This will happen if we try to incorporate type into types.
    Universe,
    /// Equivalent to `if e1 then e2 else e3`.
    IfElse {
        cond: Box<Term>,
        conseq: Box<Term>,
        alt: Box<Term>,
    },
    /// Binary expression: `e1 + e2`, `e1 - e2`, etc.
    Binary(BinaryTerm),
    /// Unary expression.
    Unary(UnaryTerm),
}

/// Term↓
#[derive(Clone, Debug, PartialEq)]
pub enum CheckableTerm {
    InfereableTerm { term: Box<Term> },
    Lambda { term: Box<CheckableTerm> },
}

#[derive(Debug, Clone)]
pub enum Value {
    VNeutral(Neutral),
    VAbs(Box<Closure<Value, EvalCtx>>),
    VUniverse,
    VPi {
        val: Box<Value>,
        body: Box<Closure<Value, EvalCtx>>, // Box<dyn Callable<Value>>
    },
}

/// A neutral term is just a variable applied to a possibly empty sequence of values or
/// is just a lambda abstraction. Neutral terms are good if we want to evalaute types on
/// the fly.
#[derive(Debug, Clone)]
pub enum Neutral {
    NVar(VariableName),
    NApp(Box<Neutral>, Box<Value>),
}

/// Some trivial literal terms.
#[derive(Clone, PartialEq)]
pub enum LitTerm {
    Int(i32),
    Bool(bool),
    Str(String),
}

#[derive(Clone, PartialEq)]
pub enum UnaryTerm {
    Not(Box<Term>),
    Neg(Box<Term>),
}

#[derive(Clone, PartialEq)]
pub enum BinaryTerm {
    Logical(BinaryLogicalExpr),
    Arith(BinaryArithmeticExpr),
}

#[derive(Clone, PartialEq)]
pub enum BinaryLogicalExpr {
    /// Addition: `e1 + e2`.
    Add((Box<Term>, Box<Term>)),
    /// Subtraction: `e1 - e2`.
    Sub((Box<Term>, Box<Term>)),
    /// Multiplication: `e1 * e2`.
    Mul((Box<Term>, Box<Term>)),
    /// Division: `e1 / e2`.
    Div((Box<Term>, Box<Term>)),
    /// Modulo: `e1 % e2`.
    Mod((Box<Term>, Box<Term>)),
}

#[derive(Clone, PartialEq)]
pub enum BinaryArithmeticExpr {
    /// Less than: `e1 < e2`.
    Lt((Box<Term>, Box<Term>)),
    /// Less than or equal to: `e1 <= e2`.
    Le((Box<Term>, Box<Term>)),
    /// Greater than: `e1 > e2`.
    Gt((Box<Term>, Box<Term>)),
    /// Greater than or equal to: `e1 >= e2`.
    Ge((Box<Term>, Box<Term>)),
    /// Equality: `e1 == e2`.
    Eq((Box<Term>, Box<Term>)),
    /// Inequality: `e1 != e2`.
    Ne((Box<Term>, Box<Term>)),
}

impl fmt::Debug for LitTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LitTerm::Int(n) => write!(f, "{}", n),
            LitTerm::Bool(b) => write!(f, "{}", b),
            LitTerm::Str(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for BinaryTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTerm::Logical(e) => write!(f, "{:?}", e),
            BinaryTerm::Arith(e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for UnaryTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryTerm::Not(e) => write!(f, "!{:?}", e),
            UnaryTerm::Neg(e) => write!(f, "-{:?}", e),
        }
    }
}

impl fmt::Debug for BinaryArithmeticExpr {
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

impl fmt::Debug for BinaryLogicalExpr {
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

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::AnnotatedTerm { term, ty } => write!(f, "{:?} : {:?}", term, ty),
            Term::App { clos, arg } => write!(f, "{:?} {:?}", clos, arg),
            Term::DependentFunctionSpace { arg, ret } => write!(f, "∀{:?}. {:?}", arg, ret),
            Term::Lit(n) => write!(f, "Lit({:?})", n),
            Term::Var(x) => write!(f, "Var({:?})", x),
            Term::Bounded(n) => write!(f, "Bounded({})", n),
            Term::Universe => write!(f, "Set"),
            Term::IfElse { cond, conseq, alt } => {
                write!(f, "if {:?} then {:?} else {:?}", cond, conseq, alt)
            }
            Term::Binary(e) => write!(f, "{:?}", e),
            Term::Unary(e) => write!(f, "{:?}", e),
        }
    }
}

impl UnaryTerm {
    pub fn extract_operand(&self) -> Box<Term> {
        match self {
            UnaryTerm::Not(e) => e.clone(),
            UnaryTerm::Neg(e) => e.clone(),
        }
    }
}

impl BinaryTerm {
    pub fn extract_operands(&self) -> (Box<Term>, Box<Term>) {
        match self {
            BinaryTerm::Logical(e) => match e {
                BinaryLogicalExpr::Add((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Sub((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Mul((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Div((e1, e2)) => (e1.clone(), e2.clone()),
                BinaryLogicalExpr::Mod((e1, e2)) => (e1.clone(), e2.clone()),
            },
            BinaryTerm::Arith(e) => match e {
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
