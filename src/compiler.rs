use std::rc::Rc;

use crate::{chunk::{Chunk, OpCode}, error::RLoxError, scanner::{self, Scanner}, token::TokenType, value::Value};


pub struct Compiler {

}

impl Compiler {
	pub fn compile(source: &str) -> Result<Chunk, RLoxError> {
		let mut scanner = Scanner::new(source);

		let mut line = 0;
		loop {
			let token = scanner.scan_token()?;

			if token.line != line {
				print!("{:04} ", token.line);
				line = token.line;
			} else {
				print!("   | ")
			}
			println!("{:02} '{}'", token.token_type as u8, token.slice(source));

			if token.token_type == TokenType::EOF {
				break
			}
		}

		Ok(chunk)
	}
}