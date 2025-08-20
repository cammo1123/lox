use crate::error::RuntimeError;
use crate::expr::{Expr, Visitor};
use crate::object::Object;
use crate::token::Token;
use std::fmt::Write;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, expr: &Expr) -> Result<String, RuntimeError> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> Result<String, RuntimeError> {
        let mut s = String::new();
        write!(&mut s, "({}", name).unwrap();

        for e in exprs {
            write!(&mut s, " {}", e.accept(self)?).unwrap();
        }

        write!(&mut s, ")").unwrap();
        Ok(s)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<String, RuntimeError> {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &Object) -> Result<String, RuntimeError> {
        Ok(value.to_string())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<String, RuntimeError> {
        Ok(name.lexeme.clone())
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<String, RuntimeError> {
        let value_str = value.accept(self)?;
        Ok(format!("(assign {} {})", name.lexeme, value_str))
    }

    fn visit_null_expr(&mut self) -> Result<String, RuntimeError> {
        Ok("nil".to_string())
    }
    
    fn visit_logical_expr( &mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
}