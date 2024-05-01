//! Defines a trait for a closures.

use std::{fmt, sync::Arc};

use crate::err::EvalResult;

#[macro_export]
macro_rules! clos {
    (lambda $arg:tt -> $body:tt, $ctx:expr) => {
        $crate::clos::Closure::new(Box::new(move |$arg| $body), $ctx)
    };
}

#[derive(Clone)]
pub struct Closure<T, C, R>
where
    T: Clone,
    C: Clone,
    R: Clone,
{
    pub f: Arc<dyn Fn(T, C) -> EvalResult<R> + Send + Sync>,
    pub ctx: C,
}

// Since closures are "hiding" the actual function, we cannot print them.
//
// One trick is to define a function that lifts back the closure into the
// raw AST node. Sometimes it is useful to define it to debug the closure.
//
// Recall what we can do in Racket using quasiquote, quote, unquote magic.
impl<T, C, R> fmt::Debug for Closure<T, C, R>
where
    T: Clone,
    C: Clone,
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Closure")
    }
}

impl<T, C, R> Closure<T, C, R>
where
    T: Clone,
    C: Clone,
    R: Clone
{
    pub fn new(f: Arc<dyn Fn(T, C) -> EvalResult<R> + Send + Sync>, ctx: C) -> Self {
        Self { f, ctx }
    }

    pub fn call(&self, x: T) -> EvalResult<R> {
        // We do not need to keep this closure: it is consumed.
        (self.f)(x, self.ctx.clone())
    }
}
