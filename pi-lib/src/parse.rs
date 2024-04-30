use std::{fs, path::Path, vec};

use crate::{
    env::EvalCtx,
    err::EvalResult,
    eval::{eval, type_check},
    term::{Value, VariableName},
};

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/lambda-pi.rs"));

pub fn eval_file<P: AsRef<Path>>(path: P) -> EvalResult<Value> {
    let f = fs::read_to_string(path.as_ref())
        .map_err(|e| crate::err::EvalError::FileNotFound(e.to_string()))?;
    let res = CmdParser::new()
        .parse(&f)
        .map_err(|e| crate::err::EvalError::ParseError(e.to_string()))?;

    let mut ctx = Default::default();
    handle_statement(res, &mut ctx)
}

pub fn handle_statement(stmt: Statement, ctx: &mut EvalCtx) -> EvalResult<Value> {
    match stmt {
        Statement::Eval(e) | Statement::Check(e) => {
            let term = ast_transform(&e, vec![])?;
            println!("debug: parsed term {term:?} with context {ctx:?}");

            type_check(0, term.clone(), ctx.clone())?;
            eval(term, ctx.clone())
        }
        Statement::Declare(ident, ty) => {
            let term = ast_transform(&ty, vec![])?;
            println!("debug: parsed term {term:?} with context {ctx:?}");

            let v = type_check(0, term, ctx.clone())?;
            ctx.0 = ctx.0.push((VariableName::Global(ident), v));

            Ok(Value::VUniverse)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parse() {
        let input = r#"
            type ___foobar :: U;
        "#;
        let input2 = r#"
            eval U;
        "#;

        let res = parse::AstParser::new().parse(input);
        let res2 = parse::AstParser::new().parse(input2);

        assert!(res.is_ok());
        assert!(res2.is_ok());
    }

    #[test]
    fn test_parse_file() {
        let path = "./test_file/test1.pi";
        let res = parse::eval_file(path);

        assert!(res.is_ok());
    }
}
