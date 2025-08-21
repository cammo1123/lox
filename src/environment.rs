use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{error::{InterpreterError, RuntimeError}, object::Object, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
	enclosing: Option<Arc<Mutex<Environment>>>,
	values: HashMap<String, Object>
}

impl Environment {
	pub fn default() -> Self {
		Self {
			enclosing: None,
			values: HashMap::new()
		}
	}

	pub fn new(enclosing: Arc<Mutex<Environment>>) -> Self {
		Self { 
			enclosing: Some(enclosing),
			values: HashMap::new() 
		}
	}

	pub fn define<S: Into<String>>(&mut self, name: S, value: Object) {
		self.values.insert(name.into(), value);
	}

	pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), InterpreterError>{
		if self.values.contains_key(&name.lexeme) {
			self.values.insert(name.lexeme.clone(), value);
			return Ok(())
		}

		if let Some(enclosing) = &self.enclosing {
			return enclosing.lock().unwrap().assign(name, value)
		}

		Err(RuntimeError {
			token: name.clone(), 
			message: format!("Undefined variable '{}'.", name.lexeme)
		}.into())
	}

	pub fn get(&self, name: &Token) -> Result<Object, InterpreterError> {
		match self.values.get(&name.lexeme) {
			Some(value) => Ok(value.clone()),
			None => {
				if let Some(enclosing) = &self.enclosing {
					return enclosing.lock().unwrap().get(name)
				}

				Err(RuntimeError {
					token: name.clone(), 
					message: format!("Undefined variable '{}'.", name.lexeme)
				}.into())
			}
		}
	}
}