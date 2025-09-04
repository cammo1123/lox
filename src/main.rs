use std::{rc::Rc};

use rlox::{chunk::{Chunk, OpCode}, value::Value, vm::VM};

pub fn main() {
	let mut chunk = Chunk::new();

	let mut constant = chunk.add_constant(Rc::new(Value::Number(1.2)));
	chunk.write(OpCode::OpConstant as u8, 123);
	chunk.write(constant as u8, 123);

	constant = chunk.add_constant(Rc::new(Value::Number(3.4)));
	chunk.write(OpCode::OpConstant as u8, 123);
	chunk.write(constant as u8 , 123);

	chunk.write(OpCode::OpAdd as u8, 123);

	constant = chunk.add_constant(Rc::new(Value::Number(5.6)));
	chunk.write(OpCode::OpConstant as u8, 123);
	chunk.write(constant as u8, 123);

	chunk.write(OpCode::OpDivide as u8, 123);
	chunk.write(OpCode::OpNegate as u8, 123);

	chunk.write(OpCode::OpReturn as u8, 123);

	let mut vm = VM::new(Rc::new(chunk));
	if let Err(val) = vm.interpret() {
		println!("{}", val)
	}
}