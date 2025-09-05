use std::{cell::RefCell, collections::HashMap, rc::Rc};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{chunk::{Chunk, OpCode}, error::{CompilerError, RLoxError}, parser::Parser, scanner::Scanner, token::{Token, TokenType}, value::{Obj, Value}};

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

type ParseFn = fn(&mut Compiler, can_assign: bool) -> Result<(), RLoxError>;
struct ParseRule {
	prefix: Option<ParseFn>,
	infix: Option<ParseFn>,
	precedence: u8
}

fn grouping_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.grouping(can_assign)
}

fn number_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.number(can_assign)
}

fn unary_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.unary(can_assign)
}

fn binary_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.binary(can_assign)
}

fn literal_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.literal(can_assign)
}

fn string_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.string(can_assign)
}

fn variable_wrapper<'src>(c: &mut Compiler<'src>, can_assign: bool) -> Result<(), RLoxError> {
    c.variable(can_assign)
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
        rules.insert(TokenType::Bang,        ParseRule { prefix: Some(unary_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::BangEqual,   ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Equality as u8 });
        rules.insert(TokenType::Equal,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::EqualEqual,  ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Equality as u8 });
        rules.insert(TokenType::Greater,     ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Comparison as u8 });
        rules.insert(TokenType::GreaterEqual,ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Comparison as u8 });
        rules.insert(TokenType::Less,        ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Comparison as u8 });
        rules.insert(TokenType::LessEqual,   ParseRule { prefix: None, infix: Some(binary_wrapper), precedence: Precedence::Comparison as u8 });
        rules.insert(TokenType::Identifier,  ParseRule { prefix: Some(variable_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::String,      ParseRule { prefix: Some(string_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Number,      ParseRule { prefix: Some(number_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::And,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Class,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Else,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::False,       ParseRule { prefix: Some(literal_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::For,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Fun,         ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::If,          ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Nil,         ParseRule { prefix: Some(literal_wrapper), infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Or,          ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Print,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Return,      ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::Super,       ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::This,        ParseRule { prefix: None, infix: None, precedence: Precedence::None as u8 });
        rules.insert(TokenType::True,        ParseRule { prefix: Some(literal_wrapper), infix: None, precedence: Precedence::None as u8 });
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
		while !self.match_token(TokenType::EOF)? {
			self.declaration()?;
		}

		self.end()?;
		Ok(!self.parser.had_error)
	}

	fn declaration(&mut self) -> Result<(), RLoxError> {
		if self.match_token(TokenType::Var)? {
			self.var_declaration()?;
		} else {
			self.statement()?;
		}

		if self.parser.panic_mode {
			self.synchronize()?;
		}

		Ok(())
	}

	fn var_declaration(&mut self) -> Result<(), RLoxError> {
		let global = self.parse_variable("Expect variable name.")?;

		if self.match_token(TokenType::Equal)? {
			self.expression()?;
		} else {
			self.emit_byte(OpCode::OpNil as u8)?;
		}

		self.consume(TokenType::SemiColon, "Expect ';' after variable declaration.")?;
		self.define_variable(global)
	}

	fn variable(&mut self, can_assign: bool) -> Result<(), RLoxError> {
		self.named_variable(&self.prev()?, can_assign)
	}

	fn named_variable(&mut self, token: &Token, can_assign: bool) -> Result<(), RLoxError> {
		let arg = self.identifier_constant(&token);

		if can_assign && self.match_token(TokenType::Equal)? {
			self.expression()?;
			self.emit_bytes(OpCode::OpSetGlobal as u8, arg)
		} else {
			self.emit_bytes(OpCode::OpGetGlobal as u8, arg)
		}
	}

	fn parse_variable(&mut self, message: &str) -> Result<u8, RLoxError> {
		self.consume(TokenType::Identifier, message)?;
		Ok(self.identifier_constant(&self.prev()?))
	}

	fn define_variable(&mut self, global: u8) -> Result<(), RLoxError> {
		self.emit_bytes(OpCode::OpDefineGlobal as u8, global)
	}

	fn identifier_constant(&mut self, name: &Token) -> u8 {
		self.make_constant(Value::obj(Obj::String(self.copy_string(name.start, name.length))))
	}

	fn synchronize(&mut self) -> Result<(), RLoxError> {
		self.parser.panic_mode = false;

		while self.curr()?.token_type != TokenType::EOF {
			if self.prev()?.token_type == TokenType::SemiColon {
				return Ok(())
			}

			match self.prev()?.token_type {
				TokenType::Class | TokenType::Fun | TokenType::Var | 
				TokenType::For | TokenType::If | TokenType::While |
				TokenType::Print | TokenType::Return => {
					return Ok(());
				}

				_ => {}
			}

			self.advance()?;
		}

		Ok(())
	}

	fn statement(&mut self) -> Result<(), RLoxError> {
		if self.match_token(TokenType::Print)? {
			self.print_statement()?;
		} else {
			self.expression_statement()?;
		}

		Ok(())
	}

	fn expression_statement(&mut self) -> Result<(), RLoxError> {
		self.expression()?;
		self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
		self.emit_byte(OpCode::OpPop as u8)
	}

	fn match_token(&mut self, token_type: TokenType) -> Result<bool, RLoxError> {
		if !self.check(token_type)? {
			return Ok(false);
		}

		self.advance()?;
		Ok(true)
	}

	fn check(&self, token_type: TokenType) -> Result<bool, RLoxError> {
		Ok(self.curr()?.token_type == token_type)
	}

	fn curr(&self) -> Result<Token, CompilerError> {
		self
			.parser
			.current
			.ok_or(CompilerError::new(0, "Current token is undefined"))
	}

	fn prev(&self) -> Result<Token, CompilerError> {
		self
			.parser
			.previous
			.ok_or(CompilerError::new(0, "Current token is undefined"))
	}

	fn print_statement(&mut self) -> Result<(), RLoxError> {
		self.expression()?;
		self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
		self.emit_byte(OpCode::OpPrint as u8)
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
		if self.curr()?.token_type == token_type {
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

	fn binary(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		let operator_type = self.prev()?.token_type;
		let rule = self.get_rule(operator_type)?;
		let new_precedence = Precedence::from_u8(rule.precedence + 1)
			.ok_or(CompilerError::new(self.prev()?.line, "Invalid Precedence"))?;
		
		self.parse_precedence(new_precedence)?;

		match operator_type {
			TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
			TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
			TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
			TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
			TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8),
			TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual as u8),
			TokenType::Greater => self.emit_byte(OpCode::OpGreater as u8),
			TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8),
			TokenType::Less => self.emit_byte(OpCode::OpLess as u8),
			TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8),
			_ => unreachable!()
		}
	}

	fn literal(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		match self.prev()?.token_type {
			TokenType::True => self.emit_byte(OpCode::OpTrue as u8),
			TokenType::Nil => self.emit_byte(OpCode::OpNil as u8),
			TokenType::False => self.emit_byte(OpCode::OpFalse as u8),
			_ => unreachable!()
		}
	}

	fn string(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		let prev = self.prev()?;
		self.emit_constant(Value::obj(Obj::String(self.copy_string(prev.start + 1, prev.length - 2))))
	}

	fn copy_string(&self, start: usize, length: usize) -> String {
		let string = &self.scanner.source[start..start + length];
		(*string).to_string()
	}

	fn get_rule(&mut self, token_type: TokenType) -> Result<&ParseRule, CompilerError> {
		self.parse_rules.get(&token_type)
			.ok_or(CompilerError::new(0, "No rule found for token type"))
	}

	fn grouping(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		self.expression()?;
		self.consume(TokenType::RightParen, "Expect ')' after expression.")
	}

	fn number(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		let prev = self.prev()?;
	
		let value = Value::Number(prev.slice(self.scanner.source).parse()
			.map_err(|e| CompilerError::new(prev.line, &format!("Unable to convert token to a number: {}", e).to_owned()))?);

	    self.emit_constant(value)
	}

	fn unary(&mut self, _can_assign: bool) -> Result<(), RLoxError> {
		let operator_type = self.prev()?.token_type;

		self.parse_precedence(Precedence::Unary)?;

		match operator_type {
			TokenType::Bang => {
				self.emit_byte(OpCode::OpNot as u8)
			}
			
			TokenType::Minus => {
				self.emit_byte(OpCode::OpNegate as u8)
			}

			_ => unreachable!()
		}
	}

	fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), RLoxError> {
		self.advance()?;

		let mut prev = self.prev()?;
		let prefix_rule = match self.get_rule(prev.token_type)?.prefix {
			Some(prefix) => prefix,
			None => return Ok(self.error("Expect expression.")),
		};

		let pre = precedence as u8;
		let can_assign = pre <= Precedence::Assignment as u8;
		prefix_rule(self, can_assign)?;

		while pre <= self.get_rule(self.curr()?.token_type)?.precedence {
			self.advance()?;
			prev = self.prev()?;

			let infix_rule =  match self.get_rule(prev.token_type)?.infix {
				Some(prefix) => prefix,
				None => return Ok(self.error("Expect expression.")),
			};

			infix_rule(self, can_assign)?;
		}

		if can_assign && self.match_token(TokenType::Equal)? {
			self.error("Invalid assignment target.");
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
		self.current_chunk.borrow_mut().write(byte as u8, self.prev()?.line);
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