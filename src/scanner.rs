use crate::{error::error, object::Object, token::{ Token, TokenType }};
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),

			start: 0,
			current: 0,
			line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}

		self.tokens.push(Token::new(TokenType::EOF, "", Object::Nil, self.line));
        self.tokens
    }

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	fn scan_token(&mut self) {
		let c: char = self.advance();

		match c {
			'\n' => self.line += 1,
			'"' => self.string(),
			'(' => self.add_token(TokenType::LeftParen),
			')' => self.add_token(TokenType::RightParen),
			'{' => self.add_token(TokenType::LeftBrace),
			'}' => self.add_token(TokenType::RightBrace),
			',' => self.add_token(TokenType::Comma),
			'.' => self.add_token(TokenType::Dot),
			'-' => self.add_token(TokenType::Minus),
			'+' => self.add_token(TokenType::Plus),
			';' => self.add_token(TokenType::SemiColon),
			'*' => self.add_token(TokenType::Star),
			'!' => {
				if self.match_str('=') {
					self.add_token(TokenType::BangEqual);
				} else {
					self.add_token(TokenType::Bang);
				}
			}
			
			'=' => {
				if self.match_str('=') {
					self.add_token(TokenType::EqualEqual);
				} else {
					self.add_token(TokenType::Equal);
				}
			}

			'<' => {
				if self.match_str('=') {
					self.add_token(TokenType::LessEqual);
				} else {
					self.add_token(TokenType::Less);
				}
			}

			'>' => {
				if self.match_str('=') {
					self.add_token(TokenType::GreaterEqual);
				} else {
					self.add_token(TokenType::Greater);
				}
			}

			'/' => {
				if self.match_str('/') {
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				} else {
					self.add_token(TokenType::Slash);
				}
			}

			' ' | '\r' | '\t' => {
				// Ignore whitespace
			}

			_ => {
				if self.is_digit(c) {
					self.number();
				} else if self.is_alpha(c) {
					self.identifier();
				} else {
					error(self.line, &format!("Unexpected character ({c})."))
				}
			}
		}
	}

	fn advance(&mut self) -> char {
		let ch = self.source[self.current..].chars().next().unwrap();
		self.current += ch.len_utf8();
		ch
	}

	fn add_token(&mut self, token_type: TokenType) {
		self.add_token_a(token_type, Object::Nil);
	}

	fn add_token_a(&mut self, token_type: TokenType, literal: Object) {
		let text = &self.source[self.start..self.current];
		self.tokens.push(Token::new(token_type, text, literal, self.line));
	}

	fn match_str(&mut self, expected: char) -> bool {
		if self.is_at_end() { 
			return false 
		}

		let next_char = self.source[self.current..].chars().next();
		if next_char != Some(expected) {
			return false;
		}

		self.current += expected.len_utf8();
		true
	}

	fn peek(&self) -> char {
		if self.is_at_end() {
			return '\0';
		}

		let next_char = self.source[self.current..].chars().next();
		if next_char == None {
			error(self.line, "Unexpected end of stream");
			return '\0';
		}

		return next_char.unwrap();
	}

	fn peek_next(&self) -> char {
		if self.current + 1 >= self.source.len() {
			return '\0';
		}

		let next_char = self.source[(self.current + 1)..].chars().next();
		if next_char == None {
			error(self.line, "Unexpected end of stream");
			return '\0';
		}

		return next_char.unwrap();
	} 

	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line += 1;
			}

			self.advance();
		}

		if self.is_at_end() {
			error(self.line, "Unterminated string.");
			return;
		}

		self.advance();

		let value = &self.source[self.start + 1..self.current - 1];
		self.add_token_a(TokenType::String, Object::String(value.to_owned()));
	}

	fn is_digit(&self, c: char) -> bool {
		c >= '0' && c <= '9'
	}

	fn number(&mut self) {
		while self.is_digit(self.peek()) {
			self.advance();
		}

		if self.peek() == '.' && self.is_digit(self.peek_next()) {
			self.advance();

			while self.is_digit(self.peek()) {
				self.advance();
			}
		}

        let value = &self.source[self.start..self.current];
		self.add_token_a(
			TokenType::Number,
			Object::Number(value.parse::<f64>().expect("Invalid number literal")),
		);
	}

	fn is_alpha(&self, c: char) -> bool {
		(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
	}

	fn is_alpha_numeric(&self, c: char) -> bool {
		self.is_alpha(c) || self.is_digit(c)
	}

	fn identifier(&mut self) {
		while self.is_alpha_numeric(self.peek()) {
			self.advance();
		}

		let text = &self.source[self.start..self.current];
		let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier);
		self.add_token(token_type.clone());
	}
}