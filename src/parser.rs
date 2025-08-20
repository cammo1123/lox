use crate::{error::ParseError, expr::Expr, object::Object, stmt::Stmt, token::{Token, TokenType}};

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
		self.assignment()
	}

	fn assignment(&mut self) -> Result<Expr, ParseError> {
		let expr = self.or()?;

		if self.match_token(vec![TokenType::Equal]) {
			let equals = self.previous();
			let value = self.assignment()?;

			if let Expr::Variable { name } = &expr {
				return Ok(Expr::Assign { 
					name: name.clone(),
					value: Box::new(value) 
				})
			}

			return Err(self.error(equals, "Invalid assignment target."));
		}

		Ok(expr)
	}

	fn or(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.and()?;

		while self.match_token(vec![TokenType::Or]) {
			let operator = self.previous();
			let right = self.and()?;
			expr = Expr::Logical { 
				left: Box::new(expr),
				operator,
				right: Box::new(right)
			}
		}

		Ok(expr)
	}

	fn and(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.equality()?;

		while self.match_token(vec![TokenType::And]) {
			let operator = self.previous();
			let right = self.equality()?;

			expr = Expr::Logical {
				left: Box::new(expr),
				operator,
				right: Box::new(right)
			};
		}

		Ok(expr)
	}

	pub fn parse(&mut self) -> Vec<Stmt> {
		let mut statements: Vec<Stmt> = Vec::new();
		while !self.is_at_end() {
			if let Some(declaration) = self.declaration() {
				statements.push(declaration);
			}
		}

		statements
	}

	fn declaration(&mut self) -> Option<Stmt> {
		let mut steps = || -> Result<Stmt, ParseError> {
			if self.match_token(vec![TokenType::Var]) {
				return self.var_declaration();
			}

			self.statement()
		};

		match steps() {
			Ok(res) => Some(res),
			Err(_) => {
				self.synchronize();
				None
			}
		}
	}

	fn statement(&mut self) -> Result<Stmt, ParseError> {
		if self.match_token(vec![TokenType::For]) {
			return self.for_statement();
		}

		if self.match_token(vec![TokenType::If]) {
			return self.if_statement();
		}

		if self.match_token(vec![TokenType::Print]) {
			return self.print_statement();
		}

		if self.match_token(vec![TokenType::While]) {
			return self.while_statement();
		}

		if self.match_token(vec![TokenType::LeftBrace]) {
			return Ok(Stmt::Block { statements: self.block()? });
		}

		self.expression_statement()
	}

	fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
		let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

		let mut initializer = Expr::Nil;
		if self.match_token(vec![TokenType::Equal]) {
			initializer = self.expression()?;
		}

		self.consume(TokenType::SemiColon, "Expect ';' after variable declaration.")?;
		Ok(Stmt::Var { name, initializer })
	}

	fn for_statement(&mut self) -> Result<Stmt, ParseError> {
		self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

		let initializer: Stmt;
		if self.match_token(vec![TokenType::SemiColon]) {
			initializer = Stmt::Nil;
		} else if self.match_token(vec![TokenType::Var]) {
			initializer = self.var_declaration()?;
		} else {
			initializer = self.expression_statement()?;
		}

		let mut condition = Expr::Nil;
		if !self.check(&TokenType::SemiColon) {
			condition = self.expression()?;
		}
		self.consume(TokenType::SemiColon, "Expect ';' after loop condition.")?;

		let mut increment = Expr::Nil;
		if !self.check(&TokenType::RightParen) {
			increment = self.expression()?;
		}
		self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

		let mut body = self.statement()?;

		if !matches!(increment, Expr::Nil) {
			body = Stmt::Block { 
				statements: vec![
					body,
					Stmt::Expression { 
						expression: increment
					}
				]
			}
		}

		if matches!(condition, Expr::Nil) {
			condition = Expr::Literal { value: Object::Bool(true) }
		}
		body = Stmt::While { condition, body: Box::new(body) };

		if !matches!(initializer, Stmt::Nil) {
			body = Stmt::Block { 
				statements:  vec![
					initializer,
					body
				]
			}
		}


		Ok(body)
	}

	fn while_statement(&mut self) -> Result<Stmt, ParseError> {
		self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
		let condition = self.expression()?;
		self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
		let body = self.statement()?;

		Ok(Stmt::While { condition, body: Box::new(body) })
	}

	fn if_statement(&mut self) -> Result<Stmt, ParseError> {
		self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
		let condition = self.expression()?;
		self.consume(TokenType::RightParen, "Expect ')' after 'if' condition.")?;

		let then_branch = self.statement()?;
		let mut else_branch = Stmt::Nil;

		if self.match_token(vec![TokenType::Else]) {
			else_branch = self.statement()?;
		}

		Ok(Stmt::If { 
			condition,
			then_branch: Box::new(then_branch),
			else_branch: Box::new(else_branch) 
		})
	}

	fn print_statement(&mut self) -> Result<Stmt, ParseError> {
		let value = self.expression()?;
		self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
		Ok(Stmt::Print { expression: value })
	}

	fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
		let expr = self.expression()?;
		self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
		Ok(Stmt::Expression { expression: expr })
	}

	fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
		let mut statements: Vec<Stmt> = Vec::new();

		while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
			if let Some(declaration) = self.declaration() {
				statements.push(declaration);
			}
		}

		self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
		Ok(statements)
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

		if self.match_token(vec![TokenType::Identifier]) {
			return Ok(Expr::Variable { name: self.previous() });
		}

		if self.match_token(vec![TokenType::LeftParen]) {
			let expression = self.expression()?;
			self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
			return Ok(Expr::Grouping { expression: Box::new(expression) });
		}

		Err(self.error(self.peek(), "Expect expression."))
	}

	fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
		if self.check(&token_type) {
			return Ok(self.advance());
		}

		Err(self.error(self.peek(), message))
	}

	fn error(&self, token: Token, message: &str) -> ParseError {
		match token.token_type {
			TokenType::EOF => ParseError::new( 
				token.line, 
				&format!("at end: {}", message) 
			),
			
			_ => ParseError::new(
				token.line, 
				&format!("at '{}' {}", token.lexeme, message) 
			)
		}
	}

	fn synchronize(&mut self) {
		println!("entering recovery for: {}", self.peek());
		self.advance();

		while !self.is_at_end() {
			if self.previous().token_type == TokenType::SemiColon {
				return;
			}

			let recover = match self.peek().token_type {
				TokenType::Class | TokenType::Fun | TokenType::Var | 
				TokenType::For | TokenType::If | TokenType::While |
				TokenType::Print | TokenType::Return => {
					true
				}

				_ => false,
			};

			println!("\tattempting to recover from: {:#?}", self.peek().token_type);
			if recover {
				return;
			}

			self.advance();
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