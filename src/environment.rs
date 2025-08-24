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

	pub fn get_at(&self, distance: usize, name: &String) -> Result<Object, InterpreterError> {
		let env_arc = self.ancestor(distance)?;
        let env = env_arc.lock().unwrap();
        match env.values.get(name) {
            Some(v) => Ok(v.clone()),
            None => Err(RuntimeError {
                token: Token::dummy(),
                message: format!("Undefined variable '{}'.", name),
            }.into()),
        }
	}

	pub fn ancestor(&self, distance: usize) -> Result<Arc<Mutex<Environment>>, InterpreterError> {
        let mut environment = match &self.enclosing {
            Some(e) => Arc::clone(e),
            None => {
                return Err(RuntimeError {
                    token: Token::dummy(),
                    message: "No enclosing environment.".to_string(),
                }
                .into())
            }
        };

        for _ in 0..distance {
            let next = {
                let guard = environment.lock().unwrap();
                match &guard.enclosing {
                    Some(e) => Arc::clone(e),
                    None => {
                        return Err(RuntimeError {
                            token: Token::dummy(),
                            message: "No enclosing environment at requested distance.".to_string(),
                        }.into())
                    }
                }
            };
            environment = next;
        }

        Ok(environment)
    }

	pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), InterpreterError> {
		if self.values.contains_key(&name.lexeme) {
			self.values.insert(name.lexeme.clone(), value.clone());
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

	pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Object) -> Result<(), InterpreterError> {
		self.ancestor(distance)?.lock()?.values.insert(name.lexeme.clone(), value.clone());
		Ok(())
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