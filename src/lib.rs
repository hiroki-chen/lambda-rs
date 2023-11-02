//! A Simply Typed Lambda Calculus interpreter with Hindley-Milner type inference.

pub mod env;
pub mod err;
pub mod expr;

#[cfg(test)]
mod tests {
    use crate::env::*;
    use crate::err::TypingError;
    use crate::expr::*;

    #[test]
    fn test_type_checking_abs() {
        let mut env = Env::empty_env();

        let expr = Expr::Abs((
            ("x".to_string(), Type::Int),
            Box::new(Expr::Var("x".to_string())),
        ));

        let ty = Type::Arrow(Box::new(Type::Int), Box::new(Type::Int));
        assert!(matches!(env.type_checking(&expr), Ok(ty)));
    }

    #[test]
    fn test_type_checking_app() {
        let mut env = Env::empty_env();

        let expr = Expr::App((
            Box::new(Expr::Abs((
                ("x".to_string(), Type::Int),
                Box::new(Expr::Var("x".to_string())),
            ))),
            Box::new(Expr::Term(1)),
        ));

        println!("{expr:?}");

        assert!(matches!(env.type_checking(&expr), Ok(Type::Int)));
    }

    #[test]
    fn test_type_checking_fail() {
        let mut env = Env::empty_env();

        let expr = Expr::App((
            Box::new(Expr::Term(4)),
            Box::new(Expr::Term(2)),
        ));

        println!("{:?}", env.type_checking(&expr));
    }
}

pub struct Parser {
    pub input: String,
    pub position: usize,
}
