use crate::{
    err::{EvalError, EvalResult},
    term::{CheckableTerm, Term, VariableName},
};

#[derive(Debug, Clone)]
pub enum Statement {
    Eval(AstNode),
    Check(AstNode),
    Declare(String, AstNode),
}

#[derive(Debug, Clone)]
pub enum Type {
    Boolean,
    Integer,
    String,
}

/// This represents the ast nodes in our core lambda calculus.
#[derive(Debug, Clone)]
pub enum AstNode {
    AnnotatedTerm {
        term: Box<AstNode>,
        ty: Box<AstNode>,
    },
    /// Basic types
    Type(Type),
    /// Applications.
    App {
        clos: Box<AstNode>,
        arg: Box<AstNode>,
    },
    Nat,
    Succ(Box<AstNode>),
    Num(usize),
    /// Variables.
    Var(String),
    Universe,
    /// Lambda abstractions.
    Lambda {
        arg: String,
        body: Box<AstNode>,
    },
    DependentFunctionSpace {
        arg: Box<AstNode>,
        ret: Box<AstNode>,
    },
}

fn ast_transform_checkable(ast: &AstNode, symbols: Vec<String>) -> EvalResult<CheckableTerm> {
    match ast {
        AstNode::Lambda { arg, body } => {
            let mut symbols = symbols.clone();
            // Add the argument to the symbols list.
            symbols.push(arg.clone());
            let body = ast_transform(body, symbols)?;

            Ok(CheckableTerm::Lambda {
                term: Box::new(CheckableTerm::InfereableTerm {
                    term: Box::new(body),
                }),
            })
        }
        _ => Ok(CheckableTerm::InfereableTerm {
            term: Box::new(ast_transform(ast, symbols)?),
        }),
    }
}

fn num_to_succ(num: usize) -> Term {
    match num {
        0 => Term::Zero,
        _ => Term::Succ {
            pred: Box::new(num_to_succ(num - 1)),
        },
    }
}

/// This function transforms the AST into a checkable term.
pub(crate) fn ast_transform(ast: &AstNode, symbols: Vec<String>) -> EvalResult<Term> {
    println!("debug: parsing {ast:?} with symbols {symbols:?}");

    match ast {
        AstNode::Universe => Ok(Term::Universe),
        AstNode::Nat => Ok(Term::Nat),
        AstNode::Succ(pred) => {
            let pred = ast_transform(pred, symbols)?;
            Ok(Term::Succ {
                pred: Box::new(pred),
            })
        }
        AstNode::AnnotatedTerm { term, ty } => {
            let t = match ast_transform(term, symbols.clone()) {
                Ok(t) => CheckableTerm::InfereableTerm { term: Box::new(t) },
                Err(_) => match ast_transform_checkable(term, symbols.clone()) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                },
            };

            let ty = ast_transform_checkable(ty, symbols)?;

            Ok(Term::AnnotatedTerm {
                term: Box::new(t),
                ty: Box::new(ty),
            })
        }
        // Get its relative index; if not, we defer it and look up in the context.
        //
        // Why don't we just return the error? This is because parsing is unaware
        // of the context, so we must defer the error to the type checking phase.
        AstNode::Var(name) => match symbols.iter().position(|x| x == name) {
            Some(index) => Ok(Term::Bounded(index)),
            None => Ok(Term::Var(VariableName::Global(name.clone()))),
        },
        AstNode::Lambda { .. } => Err(EvalError::ParseError(
            "Cannot parse single lambda abstraction.".to_string(),
        )),
        AstNode::App { clos, arg } => {
            let clos = ast_transform(clos, symbols.clone())?;
            let arg = ast_transform_checkable(arg, symbols)?;

            Ok(Term::App {
                clos: Box::new(clos),
                arg: Box::new(arg),
            })
        }
        AstNode::DependentFunctionSpace { arg, ret } => {
            let arg = ast_transform_checkable(arg, symbols.clone())?;
            let ret = ast_transform_checkable(ret, symbols)?;

            Ok(Term::DependentFunctionSpace {
                arg: Box::new(arg),
                ret: Box::new(ret),
            })
        }
        AstNode::Num(num) => Ok(Term::AnnotatedTerm {
            term: Box::new(CheckableTerm::InfereableTerm {
                term: Box::new(num_to_succ(*num)),
            }),
            ty: Box::new(CheckableTerm::InfereableTerm {
                term: Box::new(Term::Nat),
            }),
        }),
        _ => todo!("{ast:?}"),
    }
}
