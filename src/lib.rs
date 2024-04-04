//! A Simply Typed Lambda Calculus interpreter with Hindley-Milner type inference.

pub mod env;
pub mod err;
pub mod eval;
pub mod term;

#[cfg(test)]
mod tests {
    use crate::term::{LitTerm, Term};

    #[test]
    fn debug_ok() {
        let term1 = Term::Var("x".to_string());
        let term2 = Term::Lit(LitTerm::Int(42));

        assert_eq!(format!("{:?}", term1), "Var(\"x\")");
        assert_eq!(format!("{:?}", term2), "Lit(42)");
    }
}
