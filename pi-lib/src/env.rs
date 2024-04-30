//! The typing environment.

use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::term::{Value, VariableName};

/// A context is a list of variables and their values and unamed values.
#[derive(Clone, Debug)]
pub struct EvalCtx(
    pub Ctx<(VariableName, Value)>, // Actually, this is used for type checking: recall now types are terms.
    pub Ctx<Value>,                 // This part is looked up using bounded index.
);

impl EvalCtx {
    pub fn new() -> Self {
        Self(Ctx::Nil, Ctx::Nil)
    }
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new()
    }
}

/// This is a FP-like list.
#[derive(Clone)]
pub enum Ctx<T>
where
    T: Clone + fmt::Debug,
{
    Nil,
    Cons { elem: T, rest: Box<Ctx<T>> },
}

impl<T> Index<usize> for Ctx<T>
where
    T: Clone + fmt::Debug,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Ctx::Nil => panic!("Index out of bounds"),
            Ctx::Cons { elem, rest } => {
                if index == 0 {
                    elem
                } else {
                    rest.index(index - 1)
                }
            }
        }
    }
}

impl<T> IndexMut<usize> for Ctx<T>
where
    T: Clone + fmt::Debug,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Ctx::Nil => panic!("Index out of bounds"),
            Ctx::Cons { elem, rest } => {
                if index == 0 {
                    elem
                } else {
                    rest.index_mut(index - 1)
                }
            }
        }
    }
}

impl<T> fmt::Debug for Ctx<T>
where
    T: Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ctx::Nil => write!(f, "[]"),
            Ctx::Cons { elem, rest } => write!(f, "{:?} :: {:?}", elem, rest),
        }
    }
}

impl<T> Iterator for Ctx<T>
where
    T: Clone + fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Ctx::Nil => None,
            Ctx::Cons { elem, rest } => {
                let elem = elem.clone();
                *self = *rest.clone();
                Some(elem)
            }
        }
    }
}

impl<T> Ctx<T>
where
    T: Clone + fmt::Debug,
{
    pub fn lookup<F>(&self, pred: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        match self {
            Ctx::Nil => None,
            Ctx::Cons { elem, rest } => {
                if pred(elem) {
                    Some(elem.clone())
                } else {
                    rest.lookup(pred)
                }
            }
        }
    }

    pub fn push(&self, elem: T) -> Self {
        Ctx::Cons {
            elem,
            rest: Box::new(self.clone()),
        }
    }
}
