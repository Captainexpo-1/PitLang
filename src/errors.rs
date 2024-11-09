use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    UndefinedVariable(String),
    TypeError(String),
    ArgumentError(String),
    Runtime(String),
    // Add more variants as needed
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            EvalError::TypeError(msg) => write!(f, "Type error: {}", msg),
            EvalError::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
            EvalError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}
