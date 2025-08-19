use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
        }
    }
}