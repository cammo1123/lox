use crate::{error::TokenError, token::{Token, TokenType}};

#[derive(Debug)]
pub struct Scanner<'src> {
    pub source: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

	pub fn scan_token(&mut self) -> Result<Token, TokenError> {
		self.skip_whitespace()?;
		self.start = self.current;

		if self.is_at_end() {
			return Ok(self.make_token(TokenType::EOF))
		}

		let c = self.advance();

		if self.is_digit(c) {
			return self.number();
		}

		if self.is_alpha(c) {
			return self.identifier();
		}

		return match c {
			'"' => self.string(),
			'(' => return Ok(self.make_token(TokenType::LeftParen)),
			')' => return Ok(self.make_token(TokenType::RightParen)),
			'{' => return Ok(self.make_token(TokenType::LeftBrace)),
			'}' => return Ok(self.make_token(TokenType::RightBrace)),
			';' => return Ok(self.make_token(TokenType::SemiColon)),
			',' => return Ok(self.make_token(TokenType::Comma)),
			'.' => return Ok(self.make_token(TokenType::Dot)),
			'-' => return Ok(self.make_token(TokenType::Minus)),
			'+' => return Ok(self.make_token(TokenType::Plus)),
			'/' => return Ok(self.make_token(TokenType::Slash)),
			'*' => return Ok(self.make_token(TokenType::Star)),

			'!' => {
				if self.match_str('=') {
					return Ok(self.make_token(TokenType::BangEqual));
				} else {
					return Ok(self.make_token(TokenType::Bang));
				}
			}

			'=' => {
				if self.match_str('=') {
					return Ok(self.make_token(TokenType::EqualEqual));
				} else {
					return Ok(self.make_token(TokenType::Equal));
				}
			}

			'<' => {
				if self.match_str('=') {
					return Ok(self.make_token(TokenType::LessEqual));
				} else {
					return Ok(self.make_token(TokenType::Less));
				}
			}

			'>' => {
				if self.match_str('=') {
					return Ok(self.make_token(TokenType::GreaterEqual));
				} else {
					return Ok(self.make_token(TokenType::Greater));
				}
			}

			_ => Err(TokenError::new(self.line, "Unexpected character."))
		}
	}

	fn string(&mut self) -> Result<Token, TokenError> {
		while self.peek()? != Some('"') && !self.is_at_end() {
			if self.peek()? == Some('\n') {
				self.line += 1;
			}

			self.advance();
		}

		if self.is_at_end() {
			return Err(TokenError::new(self.line, "Unterminated string."));
		}

		self.advance();
		Ok(self.make_token(TokenType::String))
	}

	fn number(&mut self) -> Result<Token, TokenError> {
		while self.is_digit(self.peek()?.unwrap_or('\0')) {
			self.advance();
		}

		if self.peek()? == Some('.') && self.is_digit(self.peek_next()?.unwrap_or('\0')) {
			self.advance();

			while self.is_digit(self.peek()?.unwrap_or('\0')) {
				self.advance();
			}
		}

		Ok(self.make_token(TokenType::Number))
	}

	fn identifier(&mut self) -> Result<Token, TokenError> {
		while self.is_alpha_numeric(self.peek()?.unwrap_or('\0')) {
			self.advance();
		}

		Ok(self.make_token(self.identifier_type()?))
	}

	fn identifier_type(&self) -> Result<TokenType, TokenError> {
		match self.from_start(0)? {
			Some('a') => Ok(self.check_keyword(1, "nd", TokenType::And)),
			Some('c') => Ok(self.check_keyword(1, "lass", TokenType::Class)),
			Some('e') => Ok(self.check_keyword(1, "lse", TokenType::Else)),
			Some('i') => Ok(self.check_keyword(1, "f", TokenType::If)),
			Some('n') => Ok(self.check_keyword(1, "il", TokenType::Nil)),
			Some('o') => Ok(self.check_keyword(1, "r", TokenType::Or)),
			Some('p') => Ok(self.check_keyword(1, "rint", TokenType::Print)),
			Some('r') => Ok(self.check_keyword(1, "eturn", TokenType::Return)),
			Some('s') => Ok(self.check_keyword(1, "uper", TokenType::Super)),
			Some('v') => Ok(self.check_keyword(1, "ar", TokenType::Var)),
			Some('w') => Ok(self.check_keyword(1, "hile", TokenType::While)),
			
			Some('f') => {
				match self.from_start(1)? {
					Some('a') => return Ok(self.check_keyword(2, "lse", TokenType::False)),
					Some('o') => return Ok(self.check_keyword(2, "r", TokenType::For)),
					Some('u') => return Ok(self.check_keyword(2, "n", TokenType::Fun)),
					_ => Ok(TokenType::Identifier)
				}
			},

			Some('t') => {
				match self.from_start(1)? {
					Some('h') => return Ok(self.check_keyword(2, "is", TokenType::This)),
					Some('r') => return Ok(self.check_keyword(2, "ue", TokenType::True)),
					_ => Ok(TokenType::Identifier)
				}
			}
			
			_ => Ok(TokenType::Identifier)
		}
	}

	fn check_keyword(&self, start_offset: usize, rest: &str, token_type: TokenType) -> TokenType {
        let lexeme = &self.source[self.start+start_offset..self.current];

        if lexeme == rest {
            token_type
        } else {
            TokenType::Identifier
        }
    }

	fn skip_whitespace(&mut self) -> Result<(), TokenError> {
		loop {
			match self.peek()? {
				Some(' ' | '\r' | '\t') => { 
					self.advance(); 
				},
				
				Some('\n') => {
					self.advance();
					self.line += 1;
				}

				Some('/') => {
					if self.peek_next()? == Some('/') {
						while self.peek()? != Some('\n') && !self.is_at_end() {
							self.advance();
						}
					} else {
						return Ok(());
					}
				}

				_ => return Ok(())
			}
		}
	}

	fn is_digit(&self, c: char) -> bool {
		c >= '0' && c <= '9'
	}

	fn is_alpha(&self, c: char) -> bool {
		(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
	}

	fn is_alpha_numeric(&self, c: char) -> bool {
		self.is_alpha(c) || self.is_digit(c)
	}

	
	fn peek(&self) -> Result<Option<char>, TokenError> {
		let mut it = self.source[self.current..].chars();
		let next_char = it.next();
		Ok(next_char)
	}

	fn peek_next(&self) -> Result<Option<char>, TokenError> {
		let mut it = self.source[self.current..].chars();
		it.next();
		let next_char = it.next();
		Ok(next_char)
	}

	fn from_start(&self, start_offset: usize) -> Result<Option<char>, TokenError> {
        let mut it = self.source[self.start+start_offset..].chars();
		let next_char = it.next();
		Ok(next_char)
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

	fn advance(&mut self) -> char {
		let ch = self.source[self.current..].chars().next().unwrap_or('\0');
		self.current += ch.len_utf8();
		ch
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}

	fn make_token(&self, token_type: TokenType) -> Token {
		Token { 
			token_type, 
			start: self.start,
			length: self.current - self.start,
			line: self.line
		}
	}
}