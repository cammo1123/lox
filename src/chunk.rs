use std::{cell::RefCell, rc::Rc};

use num_derive::FromPrimitive;
use crate::value::Value;

#[derive(FromPrimitive)]
pub enum OpCode {
	OpConstant,
	OpNil,
	OpTrue,
	OpFalse,
	OpPop,
	OpDefineGlobal,
	OpGetGlobal,
	OpSetGlobal,
	OpEqual,
	OpGreater,
	OpLess,
	OpAdd,
	OpSubtract,
	OpMultiply,
	OpDivide,
	OpNot,
	OpNegate,
	OpPrint,
    OpReturn,
}

pub struct Chunk {
	pub lines: Vec<usize>,
	pub code: Rc<RefCell<Vec<u8>>>,
	pub constants: Vec<Rc<Value>>
}

impl Chunk {
	pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            code: Rc::new(RefCell::new(Vec::new())),
            constants: Vec::new(),
        }
    }

	pub fn write(&mut self, byte: u8, line: usize) {
		self.code.borrow_mut().push(byte);
		self.lines.push(line);
		assert_eq!(self.lines.len(), self.code.borrow().len());
	}

	pub fn add_constant(&mut self, value: Rc<Value>) -> usize {
		self.constants.push(value);
		self.constants.len() - 1
	}

	pub fn size(&self) -> usize {
		assert_eq!(self.lines.len(), self.code.borrow().len());
		self.lines.len()
	}
}

