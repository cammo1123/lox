use std::fmt;

#[derive(Debug)]
pub enum RLoxError {
	TokenError(TokenError),
	RuntimeError(RuntimeError),
    CompilerError(CompilerError),
}

impl fmt::Display for RLoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RLoxError::TokenError(e) => write!(f, "{}", e),
            RLoxError::RuntimeError(e) => write!(f, "{}", e),
            RLoxError::CompilerError(e) => write!(f, "{}", e),
        }
    }
}

impl From<TokenError> for RLoxError {
    fn from(e: TokenError) -> Self {
        RLoxError::TokenError(e)
    }
}

impl From<RuntimeError> for RLoxError {
    fn from(e: RuntimeError) -> Self {
        RLoxError::RuntimeError(e)
    }
}

impl From<CompilerError> for RLoxError {
    fn from(e: CompilerError) -> Self {
        RLoxError::CompilerError(e)
    }
}

#[derive(Debug)]
pub struct TokenError {
    pub line: usize,
    pub message: String,
}

impl TokenError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {} - Token error: {}", self.line, self.message)
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
        write!(f, "Line {} - Runtime error: {}", self.line, self.message)
    }
}

#[derive(Debug)]
pub struct CompilerError {
	pub line: usize,
    pub message: String,
}

impl CompilerError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {} - Compiler error: {}", self.line, self.message)
    }
}
