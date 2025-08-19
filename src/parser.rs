use std::thread::current;

use crate::{r#gen::expr::Expr, token::{Token, TokenType}};

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

	fn expression(&self) -> Expr {
		return self.equality();
	}

	fn is_at_end(&self) -> bool {
		self.peek().token_type == TokenType::EOF;
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

	fn advance(&self) {
		if self.is_at_end() {
			self.current += 1;
		}

		return self.previous();
	}

	fn match_token(&self, types: Vec<TokenType>) -> bool {
		for token_type in types.iter() {
			if self.check(token_type) {
				self.advance();
				return true;
			}
		}

		return false;
	}

	fn comparison(&self) {
		
	}

	fn equality(&self) -> Expr {
		let mut expr = self.comparison(vec![]);

		while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
			let operator = self.previous();
			let right = comparison();
			expr = Expr::Binary { left: expr, operator, right }
		}

		expr
	}
}