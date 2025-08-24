use std::{fmt, hash::{Hash, Hasher}, sync::Arc};
use ordered_float::OrderedFloat;

use crate::{error::InterpreterError, interpreter::Interpreter};

#[derive(Clone)]
pub enum Object {
    String(String),
    Number(OrderedFloat<f64>),
    Bool(bool),
    Callable(Arc<dyn Callable + Send + Sync>),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Callable(_) => write!(f, "<callable>"),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => f.debug_tuple("String").field(s).finish(),
            Object::Number(n) => f.debug_tuple("Number").field(n).finish(),
            Object::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Object::Callable(a) => {
                let ptr = Arc::as_ptr(a) as *const ();
                f.debug_tuple("Callable").field(&format_args!("{:p}", ptr)).finish()
            }
            Object::Nil => f.debug_tuple("Nil").finish(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;
        match (self, other) {
            (String(a), String(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Nil, Nil) => true,
            (Callable(a), Callable(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

// If you want Eq (empty marker), it's OK now because OrderedFloat implements Eq
impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Object::*;
        // include a discriminant tag so different variants don't collide
        std::mem::discriminant(self).hash(state);
        match self {
            String(s) => s.hash(state),
            Number(n) => n.hash(state),
            Bool(b) => b.hash(state),
            Callable(a) => {
                // hash the raw pointer address to be consistent with PartialEq (ptr equality)
                let ptr = Arc::as_ptr(a) as *const ();
                (ptr as usize).hash(state);
            }
            Nil => (),
        }
    }
}

pub trait Callable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, InterpreterError>;
}