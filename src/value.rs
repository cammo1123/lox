use std::fmt;
use std::rc::Rc;

/// The heap-allocated kinds of objects (strings for now).
#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(String),
    // Future: Function(FunctionData), Instance(InstanceData), etc.
}

impl Obj {
    pub fn is_string(&self) -> bool {
        matches!(self, Obj::String(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Obj::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    /// Concatenate two Obj::String values. Accepts references to `Rc<Obj>`
    /// (which is what Value::Obj stores). Returns `Some(Value)` when both
    /// operands are strings, otherwise `None`.
    pub fn concat_strings(a: &Rc<Obj>, b: &Rc<Obj>) -> Option<Value> {
        match (a.as_string(), b.as_string()) {
            (Some(sa), Some(sb)) => {
                Some(Value::obj(Obj::String(format!("{}{}", sa, sb))))
            }
            _ => None,
        }
    }
}

/// The VM value: small values are stored directly; bigger ones are Rc<Obj>.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Obj(Rc<Obj>),
}

impl Value {
    pub fn bool_val(b: bool) -> Self {
        Value::Bool(b)
    }
    pub fn nil() -> Self {
        Value::Nil
    }
    pub fn number(n: f64) -> Self {
        Value::Number(n)
    }
    pub fn obj(o: Obj) -> Self {
        Value::Obj(Rc::new(o))
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obj::String(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Obj(o) => write!(f, "{}", o),
        }
    }
}