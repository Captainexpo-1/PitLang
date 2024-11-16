#[derive(Debug, Clone)]
pub struct TokenizerError {
    message: String,
    line: usize,
    column: usize,
}
impl TokenizerError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
            column,
        }
    }
    pub fn as_message(&self) -> String {
        format!(
            "{} at line {} column {}",
            self.message, self.line, self.column
        )
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    message: String,
    line: usize,
    column: usize,
}

impl ParserError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
            column,
        }
    }
    pub fn as_message(&self) -> String {
        format!(
            "{} at line {} column {}",
            self.message, self.line, self.column
        )
    }
}
