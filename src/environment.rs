use std::collections::HashMap;

use crate::{error::RuntimeError, object::Object, token::Token};

#[derive(Clone)]
pub struct Environment {
	enclosing: Option<Box<Environment>>,
	values: HashMap<String, Object>
}

impl Environment {
	pub fn default() -> Self {
		Self {
			enclosing: None,
			values: HashMap::new()
		}
	}

	pub fn new(enclosing: Environment) -> Self {
		Self { 
			enclosing: Some(Box::new(enclosing)),
			values: HashMap::new() 
		}
	}

	pub fn define(&mut self, name: String, value: Object) {
		self.values.insert(name, value);
	}

	pub fn assign(&mut self, name: Token, value: Object) -> Result<(), RuntimeError>{
		if self.values.contains_key(&name.lexeme) {
			self.values.insert(name.lexeme.clone(), value);
			return Ok(())
		}

		if let Some(enclosing) = &mut self.enclosing {
			return enclosing.assign(name, value)
		}

		Err(RuntimeError::new(
			name.clone(), 
			format!("Undefined variable '{}'.", name.lexeme)
		))
	}

	pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
		match self.values.get(&name.lexeme) {
			Some(value) => Ok(value.clone()),
			None => {
				if let Some(enclosing) = &self.enclosing {
					return enclosing.get(name)
				}

				Err(RuntimeError::new(
					name.clone(), 
					format!("Undefined variable '{}'.", name.lexeme)
				))
			}
		}
	}
}