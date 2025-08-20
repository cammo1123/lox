
use crate::{error::{runtime_error, RuntimeError}, r#gen::expr::{Expr, Visitor}, object::Object, token::{Token, TokenType}};

pub struct Interpreter;
impl Interpreter {
    pub fn new() -> Self {
		Self {}
	}

    pub fn interpret(&mut self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => {
                println!("{}", self.stringify(value));
            }

            Err(err) => runtime_error(err),
        }
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn stringify(&self, object: Object) -> String {
        match object {
            Object::Nil => "nil".to_owned(),
            _ => format!("{object}"),
        }
    }
    
    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Bool(b) => b,
            _ => true
        }
    }

    fn is_equal(&self, a: Object, b: Object) -> bool {
        if let Object::Nil = a && let Object::Nil = b {
            return true;
        }

        if let Object::Nil = a {
            return false;
        }

        a.eq(&b)
    }

    fn check_number_operand(&self, operator: &Token, operand: &Object) -> Result<(), RuntimeError> {
        match operand {
            Object::Number(_) => Ok(()),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operand must be a number.",
            )),
        }
    }

    fn check_number_operands(&self, operator: &Token, operand1: &Object, operand2: &Object) -> Result<(), RuntimeError> {
        self.check_number_operand(operator, operand1)?;
        self.check_number_operand(operator, operand2)?;

        Ok(())
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, RuntimeError> {
        Ok(value.clone())
    }
    
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, RuntimeError> {
        let right = self.evaluate(right)?;

        let res = match operator.token_type {
            TokenType::Bang => Object::Bool(!self.is_truthy(right)),
            TokenType::Minus => {
                if let Object::Number(n) = right {
                    Object::Number(-n)
                } else {
                    Object::Nil
                }
            }
            _ => Object::Nil,
        };

        Ok(res)
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(l), Object::Number(r)) = (left.clone(), right.clone()) {
                    return Ok(Object::Number(l + r))
                }

                if let (Object::String(l), Object::String(r)) = (left, right) {
                    return Ok(Object::String(l + &r))
                }
                
                return Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.",
                ));
            }

            TokenType::Minus => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l + r))
                }
                unreachable!()
            }

            TokenType::Slash => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l / r))
                }
                unreachable!()
            }

            TokenType::Star => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l * r))
                }
                unreachable!()
            }

            TokenType::Greater => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l > r))
                }
                unreachable!()
            }

            TokenType::GreaterEqual => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l >= r))
                }
                unreachable!()
            }

            TokenType::Less => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l < r))
                }
                unreachable!()
            }

            TokenType::LessEqual => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l <= r))
                }
                unreachable!()
            }

            TokenType::BangEqual => Ok(Object::Bool(!self.is_equal(left, right))),
            TokenType::EqualEqual => Ok(Object::Bool(self.is_equal(left, right))),

            _ => Ok(Object::Nil)
        }
    }
}