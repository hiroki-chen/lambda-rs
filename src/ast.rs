use crate::{
    err::EvalResult,
    term::{CheckableTerm, LitTerm, Term},
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
    /// Literals.
    Lit(LitTerm),
    /// Variables.
    Var(String),
    Universe,
    /// Lambda abstractions.
    Lambda {
        arg: String,
        body: Box<AstNode>,
    },
}

/// This function transforms the AST into a checkable term.
pub(crate) fn ast_transform(ast: &AstNode) -> EvalResult<CheckableTerm> {
    match ast {
        AstNode::Universe => Ok(CheckableTerm::InfereableTerm {
            term: Box::new(Term::Universe),
        }),
        _ => todo!(),
    }
}
