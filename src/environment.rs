use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{error::RuntimeError, object::Object, token::Token};

#[derive(Clone)]
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

	pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError>{
		if self.values.contains_key(&name.lexeme) {
			self.values.insert(name.lexeme.clone(), value);
			return Ok(())
		}

		if let Some(enclosing) = &self.enclosing {
			return enclosing.lock().unwrap().assign(name, value)
		}

		Err(RuntimeError::new(
			name.clone(), 
			format!("Undefined variable '{}'.", name.lexeme)
		))
	}

	pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
		match self.values.get(&name.lexeme) {
			Some(value) => Ok(value.clone()),
			None => {
				if let Some(enclosing) = &self.enclosing {
					return enclosing.lock().unwrap().get(name)
				}

				Err(RuntimeError::new(
					name.clone(), 
					format!("Undefined variable '{}'.", name.lexeme)
				))
			}
		}
	}
}