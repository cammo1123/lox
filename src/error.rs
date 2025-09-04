use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
	ParseError(ParseError),
	RuntimeError(RuntimeError)
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::ParseError(e) => write!(f, "{}", e),
            InterpreterError::RuntimeError(e) => write!(f, "{}", e),
        }
    }
}

impl From<ParseError> for InterpreterError {
    fn from(e: ParseError) -> Self {
        InterpreterError::ParseError(e)
    }
}

impl From<RuntimeError> for InterpreterError {
    fn from(e: RuntimeError) -> Self {
        InterpreterError::RuntimeError(e)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - Parser error: {}", self.line, self.message)
    }
}

#[derive(Debug)]
pub struct RuntimeError {
	pub line: usize,
    pub message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - Runtime error: {}", self.line, self.message)
    }
}
