use std::cell::RefCell;
use std::rc::Rc;

use num_traits::FromPrimitive;

use crate::chunk::{Chunk, OpCode};
use crate::error::{InterpreterError, RuntimeError};
use crate::value::Value;

pub struct VM {
    chunk: Rc<Chunk>,
    code: Rc<RefCell<Vec<u8>>>,
    ip: usize,
	stack: Vec<Value>,
	instruction_line: usize,
}

impl VM {
    pub fn new(chunk: Rc<Chunk>) -> Self {
		VM {
			chunk: Rc::clone(&chunk),
			code: Rc::clone(&chunk.code),
			ip: 0,
			stack: Vec::with_capacity(256),
			instruction_line: 0,
		}
    }
	
	pub fn interpret(&mut self) -> Result<(), InterpreterError> {
		self.run()
	}

    fn run(&mut self) -> Result<(), InterpreterError> {
        loop {
			#[cfg(feature = "debug_trace_execution")]{
				use crate::debug::Disassemble;
				println!("{:?}", self.stack);
				Disassemble::instruction(&self.chunk, self.ip)?;
			}

			self.instruction_line = self.current_line().unwrap_or(0);
            let instruction = self.read_byte()?;
            match OpCode::from_u8(instruction) {
                Some(OpCode::OpReturn) => {
					println!("{}", self.pop()?);					
					return Ok(())
				}

				Some(OpCode::OpNegate) => {
					let value = self.pop()?;
					self.stack.push(match value {
						Value::Number(num) => Ok(Value::Number(-num)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot negate non number"))
					}?);
				}

				Some(OpCode::OpAdd) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot add two non numbers"))
					}?);
				}

				Some(OpCode::OpSubtract) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot subtract two non numbers"))
					}?);
				}

				Some(OpCode::OpDivide) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot subtract two non numbers"))
					}?);
				}

				Some(OpCode::OpMultiply) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot subtract two non numbers"))
					}?);
				}
				
				Some(OpCode::OpConstant) => {
					let constant = self.read_constant()?;
					self.stack.push(constant);
				}
                
				_ => {}
            }
        }
    }

	fn read_byte(&mut self) -> Result<u8, InterpreterError> {
		let byte = match { self.code.borrow().get(self.ip).copied() } {
			Some(x) => Ok(x),
			None => {
				let line = self.current_line().unwrap_or(0);
				Err(RuntimeError::new(line, "End of Stream"))
			},
		}?;
		self.ip += 1;
		Ok(byte)
	}

	fn read_constant(&mut self) -> Result<Value, InterpreterError>{
		let position = self.read_byte()?;
		let constant = self.chunk.constants.get(position as usize)
			.ok_or((|| {
				let line = self.current_line().unwrap_or(0);
				return RuntimeError::new(line, "Failed to get constant")
			})())?;
			
		Ok((**constant).clone())
	}

	fn current_line(&self) -> Option<usize> {
		self.chunk.lines.get(self.ip).copied()
	}

	fn pop(&mut self) -> Result<Value, RuntimeError> {
		self.stack.pop().ok_or(RuntimeError::new(self.instruction_line, "No value on stack"))
	}
}