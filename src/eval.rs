//! The evaluation rule for λΠ language.

use crate::{
    env::Ctx,
    err::{EvalError, EvalResult},
    term::{Neutral, Term, Value},
};

type EvalCtx = Ctx<(String, Value)>;

pub struct Interpreter {
    ctx: EvalCtx,
}

impl Interpreter {
    /// This is a special function that evaluates the lambda application at the value level.
    fn val_app(&mut self, clos: &Value, arg: &Value) -> EvalResult<Value> {
        match clos {
            Value::VAbs { x, body } => {
                // First we extend the environment with the argument.
                self.ctx = self.ctx.push((x.clone(), arg.clone()));
                // Then we evaluate the body of the closure.
                self.val_app(body, arg)
            }
            Value::VNeutral(n) => Ok(Value::VNeutral(Neutral::NApp(
                Box::new(n.clone()),
                Box::new(arg.clone()),
            ))),
            _ => Err(EvalError::TypeMismatch),
        }
    }

    /// Creates a new interpreter.
    pub fn new() -> Self {
        Self { ctx: Ctx::Nil }
    }

    /// Evaluates a term.
    pub fn eval(&mut self, term: &Term) -> EvalResult<Value> {
        match term {
            Term::AnnotatedTerm { term, .. } => self.eval(term),
            Term::DependentFunctionSpace { x, ty, body } => {
                let ty = self.eval(ty)?;
                let body = self.eval(body)?;
                Ok(Value::VPi {
                    x: x.clone(),
                    ty: Box::new(ty),
                    body: Box::new(body),
                })
            }
            // There are options for doing "substitution". This is the smarter one in which we
            // lookup the environment and then return the term; the other option is to do a
            // direct substitution in the body of the closure applied with the argument.
            Term::Var(x) => {
                let term = self
                    .ctx
                    .lookup(|(y, _)| x == y)
                    .ok_or(EvalError::UnboundVariable(x.clone()))?;
                Ok(term.1.clone())
            }
            Term::Abs { x, body } => Ok(Value::VAbs {
                x: x.clone(),
                body: Box::new(self.eval(body)?),
            }),
            Term::App { clos, arg } => {
                let clos = self.eval(clos)?;
                let arg = self.eval(arg)?;

                self.val_app(&clos, &arg)
            }
            Term::Universe => Ok(Value::VUniverse),
            _ => unimplemented!("not implemented yet"),
        }
    }
}
