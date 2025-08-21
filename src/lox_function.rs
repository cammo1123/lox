use std::{fmt, sync::{Arc, Mutex}};

use crate::{
    environment::Environment, error::InterpreterError, interpreter::Interpreter, object::{Callable, Object}, stmt::Stmt
};

#[derive(Debug)]
pub struct LoxFunction {
    declaration: Arc<Stmt>,
    closure: Arc<Mutex<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: Stmt, closure: Arc<Mutex<Environment>>) -> Self {
        Self {
            declaration: Arc::new(declaration),
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        match &*self.declaration {
            Stmt::Function { params, .. } => params.len(),
            _ => 0,
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, InterpreterError> {
        let new_env = Environment::new(Arc::clone(&self.closure));
        let env_arc = Arc::new(Mutex::new(new_env));

        if let Stmt::Function { params, .. } = &*self.declaration {
            let mut env_lock = env_arc.lock().unwrap();

			for (i, param) in params.iter().enumerate() {
                let name = param.lexeme.clone();
                let value = arguments.get(i).cloned().unwrap_or(Object::Nil);
                env_lock.define(name, value);
            }

			drop(env_lock);
        }

		let dec = match &*self.declaration {
			Stmt::Function { body, .. } => body,
			_ => &Vec::new(),
		};

        match interpreter.execute_block(&dec, Arc::clone(&env_arc)) {
			Ok(_) => Ok(Object::Nil),
			Err(InterpreterError::Return(return_val)) => Ok(return_val.value.clone()),
			Err(InterpreterError::Runtime(err)) => Err(err.into())
		}
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Stmt::Function { name, .. } = &*self.declaration {
            write!(f, "<fn {}>", name.lexeme)
        } else {
            write!(f, "<fn>")
        }
    }
}