use std::{cell::RefCell, collections::HashMap, rc::Rc};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{chunk::{Chunk, OpCode}, error::{CompilerError, RLoxError}, parser::Parser, scanner::Scanner, token::{Token, TokenType}, value::Value};

#[derive(FromPrimitive)]
enum Precedence {
  None,
  Assignment,  // =
  Or,          // or
  And,         // and
  Equality,    // == !=
  Comparison,  // < > <= >=
  Term,        // + -
  Factor,      // * /
  Unary,       // ! -
  Call,        // . ()
  Primary
}

pub struct Compiler<'src> {
	pub current_chunk: Rc<RefCell<Chunk>>,
	parser: Parser,
	scanner: Scanner<'src>,
	parse_rules: HashMap<TokenType, ParseRule>
}

type ParseFn = fn(&mut Compiler) -> Result<(), RLoxError>;
struct ParseRule {
	prefix: Option<ParseFn>,
	infix: Option<ParseFn>,
	precedence: u8
}

fn grouping_wrapper<'src>(c: &mut Compiler<'src>) -> Result<(), RLoxError> {
    c.grouping()
}

fn number_wrapper<'src>(c: &mut Compiler<'src>) -> Result<(), RLoxError> {
    c.number()
}

fn unary_wrapper<'src>(c: &mut Compiler<'src>) -> Result<(), RLoxError> {
    c.unary()
}

fn binary_wrapper<'src>(c: &mut Compiler<'src>) -> Result<(), RLoxError> {
    c.binary()
}

impl<'src> Compiler<'src> {
	pub fn new(source: &'src str) -> Self {
        let mut rules = HashMap::new();
        rules.insert(TokenType::LeftParen,   ParseRule { prefix: Some(grouping_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::RightParen,  ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::LeftBrace,   ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::RightBrace,  ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Comma,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Dot,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Minus,       ParseRule { prefix: Some(unary_wrapper), infix: Some(binary_wrapper), precedence: Precedence::Term as u8 });
        rules.insert(TokenType::Plus,        ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Term as u8 });
        rules.insert(TokenType::SemiColon,   ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Slash,       ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Factor as u8 });
        rules.insert(TokenType::Star,        ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Factor as u8 });
        rules.insert(TokenType::Bang,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::BangEqual,   ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Equal,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::EqualEqual,  ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Greater,     ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::GreaterEqual,ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Less,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::LessEqual,   ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Identifier,  ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::String,      ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Number,      ParseRule { prefix: Some(number_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::And,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Class,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Else,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::False,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::For,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Fun,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::If,          ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Nil,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Or,          ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Print,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Return,      ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Super,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::This,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::True,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Var,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::While,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::EOF,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });

        Self {
            current_chunk: Rc::new(RefCell::new(Chunk::new())),
            parser: Parser::new(),
            scanner: Scanner::new(source),
            parse_rules: rules,
        }
	}

	pub fn compile(&mut self) -> Result<bool, RLoxError> {
		self.parser.panic_mode = false;
		self.parser.had_error = false;

		self.advance()?;
		self.expression()?;
		self.consume(TokenType::EOF, "Expect end of expression.")?;

		self.end()?;
		Ok(!self.parser.had_error)
	}

	fn advance(&mut self) -> Result<(), RLoxError> {
		self.parser.previous = self.parser.current;

		loop {
			self.parser.current = Some(self.scanner.scan_token()?);
			if self.parser.current.is_some() {
				break;
			}

			self.error_at_current(self.parser.current
				.unwrap_or(Token { token_type: TokenType::EOF, start: 0, length: 0, line: 0 })
				.slice(self.scanner.source)
			);
		}

		Ok(())
	}

	fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), RLoxError> {
		if self.parser.current.ok_or(CompilerError::new(0, "Current token is undefined"))?.token_type == token_type {
			self.advance()?;
			return Ok(());
		}

		self.error_at_current(message);
		Ok(())
	}

	fn expression(&mut self) -> Result<(), RLoxError>{
		self.parse_precedence(Precedence::Assignment)
	} 

	fn end(&mut self) -> Result<(), RLoxError> {
		self.emit_return()?;

		#[cfg(feature = "debug_print_code")]{
			use crate::debug::Disassemble;
			Disassemble::chunk(&*self.current_chunk.borrow(), "main")?;
		}
		Ok(())
	}

	fn binary(&mut self) -> Result<(), RLoxError> {
		let prev = self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Previous token is undefined"))?;

		let operator_type = prev.token_type;
		let rule = self.get_rule(operator_type)?;
		let new_precedence = Precedence::from_u8(rule.precedence + 1)
			.ok_or(CompilerError::new(prev.line, "Invalid Precedence"))?;
		
		self.parse_precedence(new_precedence)?;

		match operator_type {
			TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
			TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
			TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
			TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
			_ => unreachable!()
		}
	}

	fn get_rule(&mut self, token_type: TokenType) -> Result<&ParseRule, CompilerError> {
		self.parse_rules.get(&token_type)
			.ok_or(CompilerError::new(0, "No rule found for token type"))
	}

	fn grouping(&mut self) -> Result<(), RLoxError> {
		self.expression()?;
		self.consume(TokenType::RightParen, "Expect ')' after expression.")
	}

	fn number(&mut self) -> Result<(), RLoxError> {
		let prev = self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Previous token is undefined"))?;
	
		let value = Value::Number(prev.slice(self.scanner.source).parse()
			.map_err(|e| CompilerError::new(prev.line, &format!("Unable to convert token to a number: {}", e).to_owned()))?);

	    self.emit_constant(value)
	}

	fn unary(&mut self) -> Result<(), RLoxError> {
		let prev = self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Previous token is undefined"))?;

		let operator_type = prev.token_type;

		self.parse_precedence(Precedence::Unary)?;

		match operator_type {
			TokenType::Minus => {
				self.emit_byte(OpCode::OpNegate as u8)
			},

			_ => unreachable!()
		}
	}

	fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), RLoxError> {
		self.advance()?;

		let mut prev = self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Previous token is undefined"))?;

		let prefix_rule = match self.get_rule(prev.token_type)?.prefix {
			Some(prefix) => prefix,
			None => return Ok(self.error("Expect expression.")),
		};

		prefix_rule(self)?;
		let pre = precedence as u8;
		while pre <= { 
			let current_token = self.parser.current.ok_or(CompilerError::new(0, "Current token is undefined"))?.token_type;
			self.get_rule(current_token)?.precedence
		} {
				self.advance()?;

				prev = self
					.parser
					.previous
					.ok_or(CompilerError::new(0, "Previous token is undefined"))?;

				let infix_rule =  match self.get_rule(prev.token_type)?.infix {
					Some(prefix) => prefix,
					None => return Ok(self.error("Expect expression.")),
				};

				infix_rule(self)?;
		}

		Ok(())
	}

	fn make_constant(&mut self, value: Value) -> u8 {
		let constant = self.current_chunk.borrow_mut().add_constant(Rc::new(value)) as u8;
		if constant > u8::MAX {
			self.error("Too many constants in one chunk.");
			return 0
		}
		constant
	}

	fn emit_constant(&mut self, value: Value) -> Result<(), RLoxError> {
		let pos = self.make_constant(value);
		self.emit_bytes(OpCode::OpConstant as u8, pos)
	}

	fn emit_byte(&mut self, byte: u8) -> Result<(), RLoxError> {
		let prev = self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Previous token is undefined"))?;
	
		self.current_chunk.borrow_mut().write(byte as u8, prev.line);
		Ok(())
	}

	fn emit_return(&mut self) -> Result<(), RLoxError> {
		self.emit_byte(OpCode::OpReturn as u8)
	}

	fn emit_bytes(&mut self, byte1: u8, byte2: u8) -> Result<(), RLoxError> {
		self.emit_byte(byte1)?;
		self.emit_byte(byte2)
	}

	fn error_at_current(&mut self, message: &str) {
		self.error_at(self.parser.current, message)
	}

	fn error(&mut self, message: &str) {
		self.error_at(self.parser.previous, message)
	}

	fn error_at(&mut self, some_token: Option<Token>, message: &str) {
		if self.parser.panic_mode {
			return;
		}

		self.parser.panic_mode = true;
		let token = some_token.unwrap_or(Token { token_type: TokenType::EOF, start: 0, length: 0, line: 0 });
		eprint!("[line {}] Error", token.line);

		if token.token_type == TokenType::EOF {
			eprint!(" at end");
		} else {
			eprint!(" at '{}'", token.slice(self.scanner.source));
		}

		eprintln!(": {}", message);
		self.parser.had_error = true;
		return;
	}
}