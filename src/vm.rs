use std::cell::RefCell;
use std::rc::Rc;

use num_traits::FromPrimitive;

use crate::chunk::{Chunk, OpCode};
use crate::compiler::Compiler;
use crate::error::{RLoxError, RuntimeError};
use crate::value::Value;

pub struct VM {
    chunk: Rc<RefCell<Chunk>>,
    code: Rc<RefCell<Vec<u8>>>,
    ip: usize,
	stack: Vec<Value>,
	instruction_line: usize,
}

impl VM {
	pub fn interpret(source: &str) -> Result<(), RLoxError> {
        let mut compiler = Compiler::new(source);
        let res = compiler.compile()?;
        let chunk = compiler.current_chunk;

        let mut vm = VM {
            chunk: Rc::clone(&chunk),
            code: Rc::clone(&chunk.borrow().code),
            ip: 0,
            stack: Vec::with_capacity(256),
            instruction_line: 0,
        };

		if res {
			vm.run()?;
		}

		Ok(())
    }

    fn run(&mut self) -> Result<(), RLoxError> {
        loop {
			#[cfg(feature = "debug_trace_execution")]{
				use crate::debug::Disassemble;
				println!("{:?}", self.stack);
				Disassemble::instruction(&*self.chunk.borrow(), self.ip)?;
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
					let res = match value {
						Value::Number(num) => Ok(Value::Number(-num)),
						_ => {
							self.stack.push(value);
							Err(RuntimeError::new(self.instruction_line, "Cannot negate non number"))
						}
					}?;
					self.stack.push(res)
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

				Some(OpCode::OpGreater) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot compare two non numbers"))
					}?);
				}

				Some(OpCode::OpLess) => {
					let b = self.pop()?;
					let a = self.pop()?;
					
					self.stack.push(match (a, b) {
						(Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
						_ => Err(RuntimeError::new(self.instruction_line, "Cannot compare two non numbers"))
					}?);
				}

				Some(OpCode::OpConstant) => {
					let constant = self.read_constant()?;
					self.stack.push(constant);
				}

				Some(OpCode::OpNil) => {
					self.stack.push(Value::Nil);
				}

				Some(OpCode::OpTrue) => {
					self.stack.push(Value::Bool(true));
				}
				
				Some(OpCode::OpFalse) => {
					self.stack.push(Value::Bool(false));
				}

				Some(OpCode::OpNot) => {
					let val = self.pop()?;
					let not = Value::Bool(self.is_falsey(&val));
					self.stack.push(not);
				}

				Some(OpCode::OpEqual) => {
					let a = self.pop()?;
					let b = self.pop()?;
					let equals = Value::Bool(self.values_equal(&a, &b));
					self.stack.push(equals);
				}
                
				_ => {}
            }
        }
    }

	fn read_byte(&mut self) -> Result<u8, RLoxError> {
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

	fn read_constant(&mut self) -> Result<Value, RLoxError>{
		let position = self.read_byte()?;
		let chunk = self.chunk.borrow();
		let constant = chunk.constants.get(position as usize)
			.ok_or((|| {
				let line = self.current_line().unwrap_or(0);
				return RuntimeError::new(line, "Failed to get constant")
			})())?;
			
		Ok((**constant).clone())
	}

	fn current_line(&self) -> Option<usize> {
		let chunk = self.chunk.borrow();
		chunk.lines.get(self.ip).copied()
	}

	fn pop(&mut self) -> Result<Value, RuntimeError> {
		self.stack.pop().ok_or(RuntimeError::new(self.instruction_line, "No value on stack"))
	}

	fn is_falsey(&mut self, value: &Value) -> bool {
		match value {
			Value::Bool(val) => !val,
			Value::Nil => true,
			_ => false
		}
	}

	fn values_equal(&mut self, a: &Value, b: &Value) -> bool {
		*a == *b
	}
}