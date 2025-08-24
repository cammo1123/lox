
use crate::{environment::Environment, error::{runtime_error, InterpreterError, ReturnValue, RuntimeError}, expr::{self, Expr, ExprAssign, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical, ExprNil, ExprUnary, ExprVariable}, lox_function::LoxFunction, object::Object, stmt::{self, Stmt, StmtBlock, StmtExpression, StmtFunction, StmtIf, StmtNil, StmtPrint, StmtReturn, StmtVar, StmtWhile}, token::{Token, TokenType}};
use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};
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
        
        Ok(Object::Number(now?.as_secs_f64().into()))
    }
}

pub struct Interpreter {
    globals: Arc<Mutex<Environment>>,
    environment: Arc<Mutex<Environment>>,
    locals: HashMap<Expr, usize>,
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
            globals: globals,
            locals: HashMap::new(),
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

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    fn look_up_variable(&self, name: &Token, expr: Expr) -> Result<Object, InterpreterError> {
        if let Some(distance) = self.locals.get(&expr) {
            self.environment.lock()?.get_at(*distance, &name.lexeme)
        } else {
            self.globals.lock()?.get(name)
        }
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
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> Result<Object, InterpreterError> {
        Ok(expr.value.clone())
    }
    
    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> Result<Object, InterpreterError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> Result<Object, InterpreterError> {
        let right = self.evaluate(&expr.right)?;

        let res = match expr.operator.token_type {
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

    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> Result<Object, InterpreterError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Plus => {
                if let (Object::Number(l), Object::Number(r)) = (left.clone(), right.clone()) {
                    return Ok(Object::Number(l + r))
                }

                if let (Object::String(l), Object::String(r)) = (left, right) {
                    return Ok(Object::String(l + &r))
                }
                
                return Err(RuntimeError {
                    token: expr.operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_owned(),
                }.into());
            }

            TokenType::Minus => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l - r))
                }
                unreachable!()
            }

            TokenType::Slash => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l / r))
                }
                unreachable!()
            }

            TokenType::Star => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Number(l * r))
                }
                unreachable!()
            }

            TokenType::Greater => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l > r))
                }
                unreachable!()
            }

            TokenType::GreaterEqual => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l >= r))
                }
                unreachable!()
            }

            TokenType::Less => {
                self.check_number_operands(&expr.operator, &left, &right)?;
                if let (Object::Number(l), Object::Number(r)) = (left, right) {
                    return Ok(Object::Bool(l < r))
                }
                unreachable!()
            }

            TokenType::LessEqual => {
                self.check_number_operands(&expr.operator, &left, &right)?;
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
    
    fn visit_variable_expr(&mut self, expr: &ExprVariable) -> Result<Object, InterpreterError> {
        self.look_up_variable(&expr.name, Expr::Variable( expr.clone() ))
    }

    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> Result<Object, InterpreterError> {
        let value = self.evaluate(&expr.value)?;

        if let Some(distance) = self.locals.get(&Expr::Assign( expr.clone() )) {
            self.environment.lock()?.assign_at(*distance, &expr.name, &value)?;
        } else {
            self.globals.lock()?.assign(&expr.name, &value)?;
        }
        
        return Ok(value);
    }
    
    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> Result<Object, InterpreterError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }
    
    fn visit_call_expr(&mut self, expr: &ExprCall) -> Result<Object, InterpreterError> {
        let callee = self.evaluate(&expr.callee)?;

        let mut evaluated_arguments: Vec<Object> = Vec::new();
        for argument in &expr.arguments {
            evaluated_arguments.push(self.evaluate(&argument)?);
        }

        match callee {
            Object::Callable(callable) => {
                if expr.arguments.len() != callable.arity() {
                    return Err(RuntimeError {
                        token: expr.paren.clone(),
                        message: format!(
                            "Expected {} arguments but got {}.",
                            callable.arity(),
                            expr.arguments.len()
                        )
                    }.into());
                }

                Ok(callable.call(self, evaluated_arguments)?)
            },

            _ => Err(RuntimeError { 
                token: expr.paren.clone(),
                message: "Can only call functions and classes.".to_owned() 
            }.into())
        }
    }

    fn visit_null_expr(&mut self, _expr: &ExprNil) -> Result<Object, InterpreterError> {
        unreachable!()
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpression) -> Result<(), InterpreterError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &StmtPrint) -> Result<(), InterpreterError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }
    
    fn visit_var_stmt(&mut self, stmt: &StmtVar) -> Result<(), InterpreterError> {
        let value = if matches!(stmt.initializer, Expr::Nil(_))  {
            Object::Nil
        } else {
            self.evaluate(&stmt.initializer)?
        };

        self.environment.lock()?.define(&stmt.name.lexeme, value);
        Ok(())
    }
    
    fn visit_block_stmt(&mut self, stmt: &StmtBlock) -> Result<(), InterpreterError> {
        let new_env_arc = Arc::new(Mutex::new(Environment::new(Arc::clone(&self.environment))));
        self.execute_block(stmt.statements.as_slice(), new_env_arc)
    }

    fn visit_if_stmt(&mut self, stmt: &StmtIf) -> Result<(), InterpreterError> {
        let condition = self.evaluate(&stmt.condition)?;
        
        if self.is_truthy(&condition) {
            self.execute(&stmt.then_branch)?;
        } else if !matches!(&*stmt.else_branch, Stmt::Nil(_)) {
            self.execute(&stmt.else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &StmtWhile) -> Result<(), InterpreterError> {
        while { let condition = self.evaluate(&stmt.condition)?; self.is_truthy(&condition) }{
            self.execute(&stmt.body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &StmtFunction) -> Result<(), InterpreterError> {
        let function = LoxFunction::new(StmtFunction { 
            name: stmt.name.clone(), 
            params: stmt.params.clone(), 
            body: stmt.body.clone() 
        }.into(), Arc::clone(&self.environment));
        
        self.environment.lock()?.define(&stmt.name.lexeme, Object::Callable(Arc::new(function)));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &StmtReturn) -> Result<(), InterpreterError> {
        let mut value = Object::Nil;

        if !matches!(stmt.value, Expr::Nil(_)) {
            value = self.evaluate(&stmt.value)?;
        }
        Err(ReturnValue { value }.into())
    }
    
    fn visit_null_stmt(&mut self, _stmt: &StmtNil) -> Result<(), InterpreterError> {
        unreachable!()
    }
}