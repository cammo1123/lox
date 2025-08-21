
use crate::{environment::Environment, error::{runtime_error, InterpreterError, ReturnValue, RuntimeError}, expr::{self, Expr}, lox_function::LoxFunction, object::Object, stmt::{self, Stmt}, token::{Token, TokenType}};
use std::{sync::{Arc, Mutex}, time::Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::object::Callable;

#[derive(Debug)]
struct NativeClock;

impl Callable for NativeClock {
    fn arity(&self) -> usize { 0 }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, InterpreterError> {
        let now: Result<Duration, InterpreterError> = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| RuntimeError {
            token: Token { token_type: TokenType::EOF, lexeme: "".to_string(), line: 0, literal: Object::Nil },
            message: format!("clock failed: {:?}", e)
        }.into());
        
        Ok(Object::Number(now?.as_secs_f64()))
    }
}

pub struct Interpreter {
    _globals: Arc<Mutex<Environment>>,
    environment: Arc<Mutex<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Arc::new(Mutex::new(Environment::default()));

        {
            let mut guard = globals.lock().unwrap();
            guard.define("clock", Object::Callable(Arc::new(NativeClock)));
        }

		Self {
            environment: Arc::clone(&globals),
            _globals: globals,
        }
	}

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements.iter() {
            match self.execute(statement) {
                Err(err) => {
                    runtime_error(err);
                    break;
                }
                _ => {}
            }
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), InterpreterError> {
        statement.accept(self)?;
        Ok(())
    }

    pub fn execute_block(&mut self, statements: &[Stmt], environment: Arc<Mutex<Environment>>) -> Result<(), InterpreterError> {
        let previous_environment = Arc::clone(&self.environment);
        self.environment = environment;

        for statement in statements.iter() {
            if let Err(err) = self.execute(statement) {
                self.environment = previous_environment;
                return Err(err);
            }
        }

        self.environment = previous_environment;
        Ok(())
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, InterpreterError> {
        expr.accept(self)
    }

    fn stringify(&self, object: Object) -> String {
        match object {
            Object::Nil => "nil".to_owned(),
            _ => format!("{object}"),
        }
    }
    
    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Bool(b) => *b,
            _ => true
        }
    }

    fn is_equal(&self, a: Object, b: Object) -> bool {
        if matches!(a, Object::Nil) && matches!(b, Object::Nil) {
            return true;
        }

        if let Object::Nil = a {
            return false;
        }

        a.eq(&b)
    }

    fn check_number_operand(&self, operator: &Token, operand: &Object) -> Result<(), InterpreterError> {
        match operand {
            Object::Number(_) => Ok(()),
            _ => Err(RuntimeError {
                token: operator.clone(),
                message: "Operand must be a number.".to_owned(),
            }.into()),
        }
    }

    fn check_number_operands(&self, operator: &Token, operand1: &Object, operand2: &Object) -> Result<(), InterpreterError> {
        self.check_number_operand(operator, operand1)?;
        self.check_number_operand(operator, operand2)?;

        Ok(())
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, InterpreterError> {
        Ok(value.clone())
    }
    
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, InterpreterError> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
        let right = self.evaluate(right)?;

        let res = match operator.token_type {
            TokenType::Bang => Object::Bool(!self.is_truthy(&right)),
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

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
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
                
                return Err(RuntimeError {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_owned(),
                }.into());
            }

            TokenType::Minus => {
                self.check_number_operands(operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l - r))
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
    
    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, InterpreterError> {
        self.environment.lock()?.get(name)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, InterpreterError> {
        let value = self.evaluate(value)?;

        self.environment.lock()?.assign(name, value.clone())?;
        return Ok(value);
    }
    
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, InterpreterError> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(right)
    }
    
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> Result<Object, InterpreterError> {
        let callee = self.evaluate(callee)?;

        let mut evaluated_arguments: Vec<Object> = Vec::new();
        for argument in arguments {
            evaluated_arguments.push(self.evaluate(argument)?);
        }

        match callee {
            Object::Callable(callable) => {
                if arguments.len() != callable.arity() {
                    return Err(RuntimeError {
                        token: paren.clone(),
                        message: format!(
                            "Expected {} arguments but got {}.",
                            callable.arity(),
                            arguments.len()
                        )
                    }.into());
                }

                Ok(callable.call(self, evaluated_arguments)?)
            },

            _ => Err(RuntimeError { 
                token: paren.clone(),
                message: "Can only call functions and classes.".to_owned() 
            }.into())
        }
    }

    fn visit_null_expr(&mut self) -> Result<Object, InterpreterError> {
        todo!()
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), InterpreterError> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), InterpreterError> {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }
    
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), InterpreterError> {
        let mut value = Object::Nil;
        
        if !matches!(initializer, Expr::Nil)  {
            value = self.evaluate(initializer)?;
        }

        self.environment.lock()?.define(&name.lexeme, value);
        Ok(())
    }
    
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), InterpreterError> {
        let new_env_arc = Arc::new(Mutex::new(Environment::new(Arc::clone(&self.environment))));
        self.execute_block(statements.as_slice(), new_env_arc)
    }

    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Stmt) -> Result<(), InterpreterError> {
        let condition = self.evaluate(condition)?;
        
        if self.is_truthy(&condition) {
            self.execute(then_branch)?;
        } else if !matches!(else_branch, Stmt::Nil) {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), InterpreterError> {
        while { let condition = self.evaluate(condition)?; self.is_truthy(&condition) }{
            self.execute(body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<(), InterpreterError> {
        let function = LoxFunction::new(Stmt::Function { name: name.clone(), params: params.clone(), body: body.clone() }, Arc::clone(&self.environment));
        self.environment.lock()?.define(&name.lexeme, Object::Callable(Arc::new(function)));
        Ok(())
    }

    fn visit_return_stmt(&mut self, _keyword: &Token, value: &Expr) -> Result<(), InterpreterError> {
        let mut eval_value = Object::Nil;

        if !matches!(value, Expr::Nil) {
            eval_value = self.evaluate(value)?;
        }

        Err(ReturnValue { value: eval_value }.into())
    }
    
    fn visit_null_stmt(&mut self) -> Result<(), InterpreterError> {
        todo!()
    }
}