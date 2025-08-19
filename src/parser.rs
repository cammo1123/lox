use crate::{r#gen::expr::Expr, object::Object, token::{Token, TokenType}};

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

impl Parser {
	pub fn new(tokens: &Vec<Token>) -> Self {
		Self {
			tokens: tokens.clone(),
			current: 0,
		}
	}

	fn expression(&mut self) -> Result<Expr, ParseError> {
		self.equality()
	}

	pub fn parse(&mut self) -> Option<Expr> {
		match self.expression() {
			Ok(expr) => Some(expr),
			Err(_) => None,
		}
	}

	fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF
	}

	fn previous(&self) -> Token {
		self.tokens.get(self.current - 1)
			.unwrap_or_else(|| panic!("Token is None"))
			.clone()
	}

	fn peek(&self) -> Token {
		self.tokens.get(self.current)
			.unwrap_or_else(|| panic!("Token is None"))
			.clone()
	}

	fn check(&self, token_type: &TokenType) -> bool {
		if self.is_at_end() {
			return false;
		}

		return self.peek().token_type == *token_type;
	}

	fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current += 1;
		}

		return self.previous();
	}

	fn match_token(&mut self, types: Vec<TokenType>) -> bool {
		for token_type in types.iter() {
			if self.check(token_type) {
				self.advance();
				return true;
			}
		}

		return false;
	}

	fn primary(&mut self) -> Result<Expr, ParseError> {
		if self.match_token(vec![TokenType::False]) {
			return Ok(Expr::Literal { value: Object::Bool(false) });
		}

		if self.match_token(vec![TokenType::True]) {
			return Ok(Expr::Literal { value: Object::Bool(true) });
		}

		if self.match_token(vec![TokenType::Nil]) {
			return Ok(Expr::Literal { value: Object::Nil });
		}

		if self.match_token(vec![TokenType::Number, TokenType::String]) {
			return Ok(Expr::Literal { value: self.previous().literal });
		}

		if self.match_token(vec![TokenType::LeftParen]) {
			let expression = self.expression()?;
			self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
			return Ok(Expr::Grouping { expression: Box::new(expression) });
		}

		self.error(self.peek(), "Expect expression.");
		Err(ParseError)
	}

	fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
		if self.check(&token_type) {
			return Ok(self.advance());
		}

		self.error(self.peek(), message);
		Err(ParseError)
	}

	fn error(&self, token: Token, message: &str) {
		use crate::error;
		if token.token_type == TokenType::EOF {
			error::error(token.line, &format!("at end: {}", message));
		} else {
			error::error(token.line, &format!("at '{}' {}", token.lexeme, message));
		}
	}

	fn unary(&mut self) -> Result<Expr, ParseError> {
		while self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
			let operator = self.previous();
			let right = self.unary()?;

			return Ok(Expr::Unary {
				operator,
				right: Box::new(right),
			});
		}

		self.primary()
	}

	fn factor(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.unary()?;

		while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
			let operator = self.previous();
			let right = self.unary()?;

			expr = Expr::Binary {
				left: Box::new(expr),
				operator,
				right: Box::new(right),
			}
		}

		Ok(expr)
	}

	fn term(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.factor()?;

		while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
			let operator = self.previous();
			let right = self.factor()?;

			expr = Expr::Binary {
				left: Box::new(expr),
				operator,
				right: Box::new(right),
			}
		}

		Ok(expr)
	}

	fn comparison(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.term()?;

		while self.match_token(vec![
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		]) {
			let operator = self.previous();
			let right = self.term()?;

			expr = Expr::Binary {
				left: Box::new(expr),
				operator,
				right: Box::new(right),
			}
		}

		Ok(expr)
	}

	fn equality(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.comparison()?;

		while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let operator = self.previous();
			let right = self.comparison()?;
			expr = Expr::Binary {
				left: Box::new(expr),
				operator,
				right: Box::new(right),
			}
		}

		Ok(expr)
	}
}