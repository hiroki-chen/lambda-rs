use std::{fs, path::Path, vec};

use crate::{
    env::TypeCtx,
    err::EvalResult,
    eval::{eval, sanity_check, type_check},
    term::{CheckableTerm, Value, VariableName},
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

pub fn handle_statement(stmt: Statement, ctx: &mut TypeCtx) -> EvalResult<Value> {
    match stmt {
        Statement::Eval(e) | Statement::Check(e) => {
            let term = ast_transform(&e, vec![])?;
            println!("debug: parsed term {term:?} with context {ctx:?}");

            type_check(0, term.clone(), ctx.clone())?;
            eval(term, ctx.clone().into())
        }
        Statement::Declare(ident, ty) => {
            let term = ast_transform(&ty, vec![])?;
            println!("debug: parsed term {term:?} with context {ctx:?}");

            type_check(0, term.clone(), ctx.clone())?;

            let ty = CheckableTerm::InfereableTerm {
                term: Box::new(term.clone()),
            };
            sanity_check(0, ty, ctx.clone(), Value::VUniverse)?;
            let v = eval(term, ctx.clone().into())?;
            ctx.1 = ctx.1.push((VariableName::Global(ident), v.clone()));

            Ok(v)
        }
        Statement::Let(ident, def) => {
            let term = ast_transform(&def, vec![])?;
            println!("debug: parsed term {term:?} with context {ctx:?}");

            let ty = type_check(0, term.clone(), ctx.clone())?;
            let v = eval(term.clone(), ctx.clone().into())?;
            ctx.0 = ctx.0.push((VariableName::Global(ident.clone()), v.clone()));
            ctx.1 = ctx.1.push((VariableName::Global(ident), ty));

            Ok(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parse() {
        let input = r#"
            def ___foobar :: U;
        "#;
        let input2 = r#"
            eval U;
        "#;

        let res = parse::CmdParser::new().parse(input);
        let res2 = parse::CmdParser::new().parse(input2);

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
