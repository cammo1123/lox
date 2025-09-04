use crate::token::Token;

pub struct Parser {
	pub previous: Option<Token>,
	pub current: Option<Token>,
	pub panic_mode: bool,
	pub had_error: bool,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			previous: None,
			current: None,
			panic_mode: false,
			had_error: false,
		}
	}
}