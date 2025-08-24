use crate::error::InterpreterError;
use crate::expr::{Expr, ExprAssign, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical, ExprNil, ExprUnary, ExprVariable, Visitor};
use std::fmt::Write;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, expr: &Expr) -> Result<String, InterpreterError> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> Result<String, InterpreterError> {
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
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> Result<String, InterpreterError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> Result<String, InterpreterError> {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> Result<String, InterpreterError> {
        Ok(expr.value.to_string())
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> Result<String, InterpreterError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<String, InterpreterError> {
        Ok(expr.name.lexeme.clone())
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<String, InterpreterError> {
        let value_str = expr.value.accept(self)?;
        Ok(format!("(assign {} {})", expr.name.lexeme, value_str))
    }

    fn visit_null_expr(&mut self, _expr: &ExprNil) -> Result<String, InterpreterError> {
        Ok("nil".to_string())
    }
    
    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<String, InterpreterError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    
    fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<String, InterpreterError> {
        Ok(format!("(call {:?} {} {:?})", expr.callee, expr.paren, expr.arguments))
    }
}