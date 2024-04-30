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
    use std::sync::Arc;

    use crate::{
        clos::Closure,
        eval::{eval_checked, lift},
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

    #[test]
    fn test_lift() {
        // \ x -> \y -> \ x
        let lambda = Value::VAbs(Box::new(Closure::new(
            Arc::new(move |x, ctx| {
                let inner = Box::new(Closure::new(Arc::new(move |_, _| Ok(x.clone())), ctx));

                Ok(Value::VAbs(inner))
            }),
            Default::default(),
        )));

        let expected = CheckableTerm::Lambda {
            term: Box::new(CheckableTerm::Lambda {
                term: Box::new(CheckableTerm::InfereableTerm {
                    term: Box::new(Term::Bounded(1)),
                }),
            }),
        };
        let lambda = lift(0, lambda);
        assert_eq!(lambda, expected);
    }
}
