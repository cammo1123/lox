use std::collections::HashMap;

use crate::{error::{InterpreterError, RuntimeError}, expr::{self, Expr, ExprAssign, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical, ExprNil, ExprVariable}, interpreter::Interpreter, stmt::{self, Stmt, StmtBlock, StmtExpression, StmtFunction, StmtIf, StmtNil, StmtPrint, StmtReturn, StmtVar, StmtWhile}, token::Token};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum FunctionType {
	None,
	Function,
}

pub struct Resolver<'a> {
	interpreter: &'a mut Interpreter,
	scopes: Vec<HashMap<String, bool>>,
	current_function: FunctionType,
}

impl<'a> Resolver<'a> {
	pub fn new(interpreter: &'a mut Interpreter) -> Self {
		Self {
			interpreter: interpreter,
			scopes: Vec::new(),
			current_function: FunctionType::None,
		}
	}

	pub fn resolve_statements(&mut self, statements: &Vec<Stmt>) -> Result<(), InterpreterError>  {
		for statement in statements {
			self.resolve_statement(&statement)?;
		}

		Ok(())
	}

	fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), InterpreterError> {
		statement.accept(self)
	}

	fn resolve_expression(&mut self, expression: &Expr) -> Result<(), InterpreterError> {
		expression.accept(self)
	}

	fn resolve_function(&mut self, stmt: &StmtFunction, function_type: FunctionType) -> Result<(), InterpreterError> {
		let enclosing_function = self.current_function.clone();
		self.current_function = function_type;
		self.begin_scope();

		for param in &stmt.params {
			self.declare(param)?;
			self.define(param)?;
		}

		self.resolve_statements(&stmt.body)?;
		self.end_scope();
		self.current_function = enclosing_function;

		Ok(())
	}

	fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), InterpreterError> {
		for i in (0..self.scopes.len()).rev() {
			if let Some(scope) = self.scopes.get(i) && scope.contains_key(&name.lexeme) {
				self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
				return Ok(())
			}
		}

		Ok(())
	}

	fn begin_scope(&mut self) {
		self.scopes.push(HashMap::new());
	}

	fn end_scope(&mut self) {
		self.scopes.pop();
	}

	fn declare(&mut self, name: &Token) -> Result<(), InterpreterError> {
		if self.scopes.is_empty() {
			return Ok(())
		}

		if let Some(scope) = self.scopes.last_mut() {
			if scope.contains_key(&name.lexeme) {
				return Err(RuntimeError {
					token: name.clone(),
					message: "Already a variable with this name in this scope.".to_owned()
				}.into())
			}
			scope.insert(name.lexeme.clone(), false);
		}

		Ok(())
	}

	fn define(&mut self, name: &Token) -> Result<(), InterpreterError> {
		if self.scopes.is_empty() {
			return Ok(())
		}

		if let Some(scope) = self.scopes.last_mut() {
			scope.insert(name.lexeme.clone(), true);
		} else {
			return Err(RuntimeError {
				token: name.clone(),
				message: "Failed to insert into scope".to_owned()
			}.into());
		}

		Ok(())
	}
}


impl<'a> expr::Visitor<()> for Resolver<'a> {
	fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.value)?;
		self.resolve_local(&Expr::Assign( expr.clone() ), &expr.name)
	}
	
	fn visit_binary_expr(&mut self, expr: &ExprBinary) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.left)?;
		self.resolve_expression(&expr.right)
	}
	
	fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.callee)?;

		for argument in &expr.arguments {
			self.resolve_expression(argument)?;
		}

		Ok(())
	}
	
	fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.expression)
	}
	
	fn visit_literal_expr(&mut self, _expr: &ExprLiteral) -> Result<(), InterpreterError> {
		Ok(())
	}
	
	fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.left)?;
		self.resolve_expression(&expr.right)
	}

	fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<(), InterpreterError> {
		if !self.scopes.is_empty() {
			if let Some(scope) = self.scopes.last_mut() && let Some(val) = scope.get(&expr.name.lexeme) {
				if val == &false {
					return Err(RuntimeError {
						token: expr.name.clone(),
						message: "Can't read local variable in its own initializer.".to_owned(),
					}.into());
				}
			}
		}

		self.resolve_local(&Expr::Variable(expr.clone()), &expr.name)
	}
	
	fn visit_unary_expr(&mut self, expr: &expr::ExprUnary) -> Result<(), InterpreterError> {
		self.resolve_expression(&expr.right)
	}

	fn visit_null_expr(&mut self, _expr: &ExprNil) -> Result<(), InterpreterError> {
		unreachable!()
	}
}


impl<'a> stmt::Visitor<()> for Resolver<'a> {
	fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> Result<(), crate::error::InterpreterError> {
		self.begin_scope();
		self.resolve_statements(&stmt.statements)?;
		self.end_scope();
		Ok(())
	}

	fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> Result<(), crate::error::InterpreterError> {
		self.resolve_expression(&stmt.expression)
	}

	fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> Result<(), crate::error::InterpreterError> {
		self.declare(&stmt.name)?;
		self.define(&stmt.name)?;

		self.resolve_function(stmt, FunctionType::Function)
	}

	fn visit_if_stmt(&mut self, stmt: &StmtIf) -> Result<(), crate::error::InterpreterError> {
		self.resolve_expression(&stmt.condition)?;
		self.resolve_statement(&stmt.then_branch)?;

		if !matches!(&*stmt.else_branch, Stmt::Nil(_)) {
			self.resolve_statement(&stmt.else_branch)?;
		}

		Ok(())
	}

	fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> Result<(), crate::error::InterpreterError> {
		self.resolve_expression(&stmt.expression)
	}

	fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> Result<(), crate::error::InterpreterError> {
		if self.current_function == FunctionType::None {
			return Err(RuntimeError {
				token: stmt.keyword.clone(),
				message: "Can't return from top-level code.".to_owned(),
			}.into())
		}

		if !matches!(stmt.value, Expr::Nil(_)) {
			self.resolve_expression(&stmt.value)?;
		}

		Ok(())
	}

	fn visit_var_stmt(&mut self, stmt: &StmtVar) -> Result<(), crate::error::InterpreterError> {
		self.declare(&stmt.name)?;

		if !matches!(stmt.initializer, Expr::Nil(_)) {
			self.resolve_expression(&stmt.initializer)?
		}

		self.define(&stmt.name)
	}

	fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> Result<(), crate::error::InterpreterError> {
		self.resolve_expression(&stmt.condition)?;
		self.resolve_statement(&stmt.body)
	}

	fn visit_null_stmt(&mut self, _stmt: &StmtNil) -> Result<(), crate::error::InterpreterError> {
		unreachable!()
	}
}
