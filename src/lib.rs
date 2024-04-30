//! A Simply Typed Lambda Calculus interpreter with Hindley-Milner type inference.

pub mod ast;
pub mod clos;
pub mod env;
pub mod err;
pub mod eval;
pub mod parse;
pub mod term;

#[cfg(test)]
mod tests {
    use crate::{
        eval::eval_checked,
        term::{CheckableTerm, Term, Value},
    };

    #[test]
    fn test_id() {
        // \ x -> x
        let identity = CheckableTerm::Lambda {
            term: Box::new(CheckableTerm::InfereableTerm {
                term: Box::new(Term::Bounded(0)),
            }),
        };

        let res = eval_checked(identity, Default::default());

        assert!(matches!(res, Ok(Value::VAbs(_))));
    }
}
