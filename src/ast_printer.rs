use crate::r#gen::expr::{Expr, Visitor};
use crate::object::Object;
use crate::token::Token;
use std::fmt::Write;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
		AstPrinter
	}

    pub fn print(&mut self, expr: &Expr) -> String {
		expr.accept(self)
	}

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut s = String::new();
        write!(&mut s, "({}", name).unwrap();

        for e in exprs {
			write!(&mut s, " {}", e.accept(self)).unwrap(); 
		}
        
		write!(&mut s, ")").unwrap();
        s
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }
    fn visit_literal_expr(&mut self, value: &Object) -> String {
        value.to_string()
    }
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}