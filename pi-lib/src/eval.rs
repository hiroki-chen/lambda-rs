//! The evaluation rule for λΠ language.

use std::sync::Arc;

use crate::{
    clos::Closure,
    env::EvalCtx,
    err::{EvalError, EvalResult},
    term::{CheckableTerm, Neutral, Term, Type, Value, VariableName},
};

fn lift_neutral(de_brujin_index: usize, n: Neutral) -> Term {
    match n {
        Neutral::NApp(clos, arg) => Term::App {
            clos: Box::new(lift_neutral(de_brujin_index, *clos)),
            arg: Box::new(lift(de_brujin_index, *arg)),
        },
        Neutral::NVar(name) => match name {
            // Bounded.
            VariableName::Quote(idx) => Term::Bounded(de_brujin_index - idx - 1),
            _ => Term::Var(name),
        },
    }
}

/// Lift back a value into a term.
pub(crate) fn lift(de_brujin_index: usize, val: Value) -> CheckableTerm {
    match val {
        Value::VAbs(clos) => {
            let body = clos
                .call(Value::VNeutral(Neutral::NVar(VariableName::Quote(
                    de_brujin_index,
                ))))
                .expect("closure call failed");
            CheckableTerm::Lambda {
                term: Box::new(lift(de_brujin_index + 1, body)),
            }
        }
        Value::VNeutral(n) => CheckableTerm::InfereableTerm {
            term: Box::new(lift_neutral(de_brujin_index, n)),
        },
        Value::VUniverse => CheckableTerm::InfereableTerm {
            term: Box::new(Term::Universe),
        },
        Value::VPi { val, body } => {
            let arg = lift(de_brujin_index, *val);
            let body = body
                .call(Value::VNeutral(Neutral::NVar(VariableName::Quote(
                    de_brujin_index,
                ))))
                .expect("closure call failed");
            CheckableTerm::InfereableTerm {
                term: Box::new(Term::DependentFunctionSpace {
                    arg: Box::new(arg),
                    ret: Box::new(lift(de_brujin_index + 1, body)),
                }),
            }
        }
        Value::VZero => CheckableTerm::Zero,
        Value::VSucc { pred } => CheckableTerm::Succ {
            term: Box::new(lift(de_brujin_index, *pred)),
        },
        Value::VNat => CheckableTerm::InfereableTerm {
            term: Box::new(Term::Nat),
        },
    }
}

fn subst(de_brujin_index: usize, t_what: Term, t_for: Term) -> Term {
    match t_for {
        Term::AnnotatedTerm { term, ty } => {
            // Subsitute all.
            let term = Box::new(subst_checked(de_brujin_index, t_what.clone(), *term));
            let ty = Box::new(subst_checked(de_brujin_index, t_what, *ty));
            Term::AnnotatedTerm { term, ty }
        }
        Term::Bounded(idx) => match idx == de_brujin_index {
            true => t_what,
            false => Term::Bounded(idx),
        },
        Term::Var(name) => Term::Var(name),
        Term::App { clos, arg } => Term::App {
            clos: Box::new(subst(de_brujin_index, t_what.clone(), *clos)),
            arg: Box::new(subst_checked(de_brujin_index, t_what, *arg)),
        },
        Term::Universe => Term::Universe,
        Term::DependentFunctionSpace { arg, ret } => {
            let arg = Box::new(subst_checked(de_brujin_index, t_what.clone(), *arg));
            let ret = Box::new(subst_checked(de_brujin_index + 1, t_what, *ret));
            Term::DependentFunctionSpace { arg, ret }
        }
        Term::Nat => Term::Nat,
        Term::Succ { pred } => {
            let pred = Box::new(subst(de_brujin_index, t_what, *pred));
            Term::Succ { pred }
        }
        _ => todo!("not implemented yet for {t_for:?}"),
    }
}

fn subst_checked(de_brujin_index: usize, t_what: Term, t_for: CheckableTerm) -> CheckableTerm {
    match t_for {
        CheckableTerm::InfereableTerm { term } => CheckableTerm::InfereableTerm {
            term: Box::new(subst(de_brujin_index, t_what, *term)),
        },
        CheckableTerm::Lambda { term } => CheckableTerm::Lambda {
            term: Box::new(subst_checked(de_brujin_index + 1, t_what, *term)),
        },
        CheckableTerm::Succ { term } => CheckableTerm::Succ {
            term: Box::new(subst_checked(de_brujin_index, t_what, *term)),
        },
        CheckableTerm::Zero => CheckableTerm::Zero,
    }
}

/// This is a special function that evaluates the lambda application at the value level.
fn val_app(clos: &Value, arg: &Value) -> EvalResult<Value> {
    match clos {
        Value::VAbs(clos) => clos.call(arg.clone()),
        Value::VNeutral(n) => Ok(Value::VNeutral(Neutral::NApp(
            Box::new(n.clone()),
            Box::new(arg.clone()),
        ))),
        _ => Err(EvalError::TypeMismatch(format!(
            "Cannot apply a non-function value: {:?}",
            clos
        ))),
    }
}

pub fn eval_checked(term: CheckableTerm, ctx: EvalCtx) -> EvalResult<Value> {
    match term {
        // May cause some non-terminating loops.
        CheckableTerm::InfereableTerm { term } => eval(*term, ctx),
        //
        CheckableTerm::Lambda { term } => {
            // We move the contexts into the closure.
            let f = move |x, mut ctx: EvalCtx| {
                ctx.1 = ctx.1.push(x);
                eval_checked(*term.clone(), ctx)
            };

            Ok(Value::VAbs(Box::new(Closure::new(Arc::new(f), ctx))))
        }
        CheckableTerm::Succ { term } => {
            let pred = eval_checked(*term, ctx)?;
            Ok(Value::VSucc {
                pred: Box::new(pred),
            })
        }
        CheckableTerm::Zero => Ok(Value::VZero),
    }
}

/// Evaluates a term: term -> context -> Result(Value)
///
/// All the values are moved. The tricky part of the Rust ownership model is that
/// we cannot make closures cloneable while keeping the closure type-erased. If we
/// just borrow `term` and `ctx`, we will have to make sure that the closure does not
/// outlive the `term` and `ctx` (which in fact will).
///
/// We simply clone everything to ensure that the closure is self-contained.
pub fn eval(term: Term, ctx: EvalCtx) -> EvalResult<Value> {
    match term {
        // Type erasure: we do not need to keep the annotation.
        Term::AnnotatedTerm { term, .. } => eval_checked(*term, ctx),
        Term::DependentFunctionSpace { arg, ret } => {
            let val = eval_checked(*arg, ctx.clone())?;
            // Let us move `ret` into the closure's evaluation context.
            let body = move |x, mut ctx: EvalCtx| {
                ctx.1 = ctx.1.push(x);
                // A'
                eval_checked(*ret.clone(), ctx)
            };

            Ok(Value::VPi {
                val: Box::new(val),
                body: Box::new(Closure::new(Arc::new(body), ctx)),
            })
        }
        Term::Var(x) => Ok(Value::VNeutral(Neutral::NVar(x.clone()))),
        // Try to look up the context and get the result.
        Term::Bounded(idx) => match ctx.1.into_iter().nth(idx) {
            Some(val) => Ok(val),
            None => Err(EvalError::UnboundVariable(format!(
                "Variable at index {} is not found in the context",
                idx
            ))),
        },
        Term::App { clos, arg } => {
            let clos = eval(*clos, ctx.clone())?;
            let arg = eval_checked(*arg, ctx.clone())?;

            val_app(&clos, &arg)
        }
        // Universe does not evaluate to anything.
        Term::Universe => Ok(Value::VUniverse),
        Term::Zero => Ok(Value::VZero),
        Term::Nat => Ok(Value::VNat),
        Term::Succ { pred } => {
            let pred = eval(*pred, ctx)?;
            Ok(Value::VSucc {
                pred: Box::new(pred),
            })
        }
        _ => unimplemented!("not implemented yet for {term:?}"),
    }
}

/// Do a type check.
pub fn type_check(de_brujin_index: usize, term: Term, mut ctx: EvalCtx) -> EvalResult<Type> {
    match term {
        Term::AnnotatedTerm { term, ty } => {
            // Ensure that the type is a universe.
            sanity_check(de_brujin_index, *ty.clone(), ctx.clone(), Value::VUniverse)?;
            // Evaluate that type.
            let ty = eval_checked(*ty, Default::default())?;
            // Then do the type checking.
            sanity_check(de_brujin_index, *term, ctx, ty.clone()).map(|_| ty)
        }
        Term::Universe => Ok(Value::VUniverse),
        Term::DependentFunctionSpace { arg, ret } => {
            println!("debug: checking {arg:?} -> {ret:?}");

            sanity_check(de_brujin_index, *arg.clone(), ctx.clone(), Value::VUniverse)?;
            let arg_ty = eval_checked(*arg, Default::default())?;
            let substituted =
                subst_checked(0, Term::Var(VariableName::Local(de_brujin_index)), *ret);
            // We push the variable into the context.
            ctx.0 = ctx.0.push((VariableName::Local(de_brujin_index), arg_ty));
            sanity_check(de_brujin_index + 1, substituted, ctx, Value::VUniverse)?;
            // Size ↑ ?
            Ok(Value::VUniverse)
        }
        Term::Var(name) => match ctx.0.into_iter().find(|(n, _)| n == &name) {
            Some((_, val)) => Ok(val),
            None => Err(EvalError::UnboundVariable(format!(
                "Variable {:?} is not found in the context",
                name
            ))),
        },
        Term::App { clos, arg } => {
            let ty = type_check(de_brujin_index, *clos.clone(), ctx.clone())?;

            if let Value::VPi { val, body } = ty {
                // Let us check if the argument is of the right type.
                sanity_check(de_brujin_index, *arg.clone(), ctx, *val)?;

                let arg = eval_checked(*arg, Default::default())?;
                body.call(arg)
            } else {
                Err(EvalError::TypeMismatch(format!(
                    "Expected a dependent function, found {:?}",
                    ty
                )))
            }
        }
        Term::Nat => Ok(Value::VUniverse),
        Term::Zero => Ok(Value::VNat),
        Term::Succ { pred } => {
            let pred_ty = type_check(de_brujin_index, *pred.clone(), ctx)?;
            match pred_ty {
                Value::VNat => Ok(Value::VNat),
                _ => Err(EvalError::TypeMismatch(format!(
                    "Expected a natural number, found {:?}",
                    pred_ty
                ))),
            }
        }
        _ => todo!("not implemented yet for {term:?}"),
    }
}

/// Nothing is returned since the type is already know. We only check if such type formations are valid.
pub fn sanity_check(
    de_brujin_index: usize,
    term: CheckableTerm,
    mut ctx: EvalCtx,
    ty: Type,
) -> EvalResult<()> {
    match term {
        CheckableTerm::Zero => Ok(()),
        CheckableTerm::InfereableTerm { term } => {
            let val = type_check(de_brujin_index, *term, ctx)?;

            let lhs = lift(0, val);
            let rhs = lift(0, ty);

            if lhs != rhs {
                Err(EvalError::TypeMismatch(format!(
                    "Type mismatch: expected {:?}, found {:?}",
                    rhs, lhs
                )))
            } else {
                Ok(())
            }
        }
        CheckableTerm::Lambda { term } => {
            match ty {
                Value::VPi { val, body } => {
                    let substituted = subst_checked(
                        0,
                        Term::Var(VariableName::Local(de_brujin_index)),
                        *term.clone(),
                    );

                    // We push the variable into the context.
                    ctx.0 = ctx.0.push((VariableName::Local(de_brujin_index), *val));
                    sanity_check(
                        de_brujin_index + 1,
                        substituted,
                        ctx,
                        body.call(Value::VNeutral(Neutral::NVar(VariableName::Local(
                            de_brujin_index,
                        ))))?,
                    )?;
                    Ok(())
                }
                _ => Err(EvalError::TypeMismatch(format!(
                    "Expected a dependent function, found {:?}",
                    ty
                ))),
            }
        }
        CheckableTerm::Succ { term } => {
            let val = eval_checked(*term, Default::default())?;
            match val {
                Value::VZero => Ok(()),
                Value::VSucc { pred } => {
                    let predl = lift(de_brujin_index, *pred);
                    let predr = lift(de_brujin_index, Value::VNat);
                    if predl == predr {
                        Ok(())
                    } else {
                        Err(EvalError::TypeMismatch(format!(
                            "Type mismatch: expected {:?}, found {:?}",
                            predr, predl
                        )))
                    }
                }
                _ => Err(EvalError::TypeMismatch(
                    "Expected a natural number or a successor, found {val:?}".to_string(),
                )),
            }
        }
    }
}
