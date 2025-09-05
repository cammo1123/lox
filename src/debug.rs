use std::usize;

use num_traits::FromPrimitive;

use crate::{chunk::{Chunk, OpCode}, error::{RLoxError, RuntimeError}};

pub struct Disassemble {

}

impl Disassemble {
	pub fn chunk(chunk: &Chunk, name: &str) -> Result<(), RLoxError>  {
		println!("== {} ==", name);

		let mut offset = 0usize;
		while offset < chunk.size() {
			offset = Self::instruction(chunk, offset)?;
		}

		Ok(())
	}

	pub fn instruction(chunk: &Chunk, offset: usize) -> Result<usize, RLoxError> {
		print!("{:04} ", offset);

		
		let line = chunk.lines.get(offset).ok_or(RuntimeError::new(0, &format!("Failed to get line for {}", offset).to_owned()))?;
		if offset > 0 {
			let prev_line = chunk.lines.get(offset - 1).ok_or(RuntimeError::new(*line, &format!("Failed to get line for {}", offset).to_owned()))?;

			if line == prev_line {
				print!("   | ");
			} else {
				print!("{:04} ", line);
			}
		} else {
			print!("{:04} ", line);
		}

		let code = chunk.code.borrow();
		let instruction = code.get(offset)
				.ok_or(RuntimeError::new(*line, &format!("Failed to instruction on line {}.", offset).to_owned()))?;

		return match OpCode::from_u8(*instruction) {
			Some(OpCode::OpReturn) => Ok(Self::simple_instruction("OpReturn", offset)?),
			Some(OpCode::OpNegate) => Ok(Self::simple_instruction("OpNegate", offset)?),
			Some(OpCode::OpNot) => Ok(Self::simple_instruction("OpNot", offset)?),
			Some(OpCode::OpAdd) => Ok(Self::simple_instruction("OpAdd", offset)?),
			Some(OpCode::OpSubtract) => Ok(Self::simple_instruction("OpSubtract", offset)?),
			Some(OpCode::OpDivide) => Ok(Self::simple_instruction("OpDivide", offset)?),
			Some(OpCode::OpMultiply) => Ok(Self::simple_instruction("OpMultiply", offset)?),
			Some(OpCode::OpConstant) => Ok(Self::constant_instruction("OpConstant", chunk, offset)?),
			Some(OpCode::OpNil) => Ok(Self::simple_instruction("OpNil", offset)?),
			Some(OpCode::OpTrue) => Ok(Self::simple_instruction("OpTrue", offset)?),
			Some(OpCode::OpFalse) => Ok(Self::simple_instruction("OpFalse", offset)?),
			Some(OpCode::OpEqual) => Ok(Self::simple_instruction("OpEqual", offset)?),
			Some(OpCode::OpLess) => Ok(Self::simple_instruction("OpLess", offset)?),
			Some(OpCode::OpGreater) => Ok(Self::simple_instruction("OpGreater", offset)?),
			_ => {
				println!("Unknown opcode {}", instruction);
				return Ok(offset + 1);
			}
		}
	}

	fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> Result<usize, RuntimeError> {
		let code = chunk.code.borrow();
		let constant = code.get(offset + 1).ok_or(RuntimeError::new(0, "message"))?;
		let value = chunk.constants.get(*constant as usize).ok_or(RuntimeError::new(0, "message"))?;
		println!("{:<16} {:04} '{}'", name, constant, value);
		Ok(offset + 2)
	}

	fn simple_instruction(name: &str, offset: usize) -> Result<usize, RuntimeError> {
		println!("{}", name);
		Ok(offset + 1)
	}
}