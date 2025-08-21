use crate::{object::Object, token::{Token, TokenType}};
use std::{fmt, sync::{atomic::{AtomicBool, Ordering}, PoisonError}};

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);
pub static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error(line: usize, message: &str) {
	report(line, "", message);
}

pub fn runtime_error(error: RuntimeError) {
	eprintln!("{}\n[line {}]", error.message, error.token.line);
	HAD_RUNTIME_ERROR.store(true, Ordering::SeqCst);
}

fn report(line: usize, where_: &str, message: &str) {
	eprintln!("[line {line}] Error{where_}: {message}");
	HAD_ERROR.store(true, Ordering::SeqCst);
}	

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::SeqCst)
}

pub fn had_runtime_error() -> bool {
    HAD_RUNTIME_ERROR.load(Ordering::SeqCst)
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        Self {
            token,
            message: message.into(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

impl<T> From<PoisonError<T>> for RuntimeError {
    fn from(err: PoisonError<T>) -> Self {
        RuntimeError::new(
            Token { 
                token_type: TokenType::EOF, 
                lexeme: "".to_owned(),
                line: 0,
                literal: Object::Nil
            },
            format!("mutex poisoned: {:?}", err),
        )
    }
}

impl std::error::Error for RuntimeError {}