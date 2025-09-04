use std::fmt;
// use std::rc::Rc;

/// The heap-allocated kinds of objects (strings for now).
// #[derive(Debug, Clone, PartialEq)]
// pub enum Obj {
//     String(String),
    // Future: Function(FunctionData), Instance(InstanceData), etc.
// }

/// The VM value: small values are stored directly; bigger ones are Rc<Obj>.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Bool(bool),
    // Nil,
    Number(f64),
    // Obj(Rc<Obj>),
}

impl Value {
    // constructors
    // pub fn bool_val(b: bool) -> Self { Value::Bool(b) }
    // pub fn nil() -> Self { Value::Nil }
    pub fn number(n: f64) -> Self { Value::Number(n) }
    // pub fn obj(o: Obj) -> Self { Value::Obj(Rc::new(o)) }

    // helpers used by the VM
    // pub fn is_string(&self) -> bool {
    //     matches!(self, Value::Obj(o) if matches!(&**o, Obj::String(_)))
    // }

    // pub fn as_string(&self) -> Option<&str> {
    //     if let Value::Obj(o) = self {
    //         if let Obj::String(s) = &**o {
    //             return Some(s);
    //         }
    //     }
    //     None
    // }

    // cloning cheap because Rc is cheap to clone
    // pub fn concat_strings(a: &Value, b: &Value) -> Option<Value> {
    //     match (a.as_string(), b.as_string()) {
    //         (Some(sa), Some(sb)) => Some(Value::obj(Obj::String(format!("{}{}", sa, sb)))),
    //         _ => None,
    //     }
    // }
}

// impl fmt::Display for Obj {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Obj::String(s) => write!(f, "{}", s),
//         }
//     }
// }

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Value::Bool(b) => write!(f, "{}", b),
            // Value::Nil => write!(f, "nil"),
            Value::Number(n) => {
                // mimic C's %g-ish printing: let Rust do its default formatting
                write!(f, "{}", n)
            }
            // Value::Obj(o) => write!(f, "{}", o),
        }
    }
}