use crate::common::TokenizerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Number,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Function,
    If,
    Else,
    Return,
    Identifier,
    Let,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Assign,
    SemiColon,
    NewLine,
    Comma,
    Dot,
    Colon,
    Null,
    True,
    False,
    While,
    For,
    Mod,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Inc,
    Dec,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, value: String, line: usize, column: usize) -> Token {
        Token {
            kind,
            value,
            line,
            column,
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c.is_ascii_digit()
}

fn get_identifier(id: String, line: usize, column: usize) -> Token {
    match id.as_str() {
        "let" => Token::new(TokenKind::Let, id, line, column),
        "fn" => Token::new(TokenKind::Function, id, line, column),
        "if" => Token::new(TokenKind::If, id, line, column),
        "else" => Token::new(TokenKind::Else, id, line, column),
        "return" => Token::new(TokenKind::Return, id, line, column),
        "null" => Token::new(TokenKind::Null, id, line, column),
        "true" => Token::new(TokenKind::True, id, line, column),
        "false" => Token::new(TokenKind::False, id, line, column),
        "while" => Token::new(TokenKind::While, id, line, column),
        "for" => Token::new(TokenKind::For, id, line, column),

        _ => Token::new(TokenKind::Identifier, id, line, column),
    }
}

fn error(message: &str, line: usize, column: usize) -> Result<Vec<Token>, TokenizerError> {
    Err(TokenizerError::new(message, line, column))
}

pub fn tokenize(text: String) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut chars = text.chars().peekable();

    let mut line: usize = 1;
    let mut col: usize = 1;
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        value.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenKind::Number, value, line, col));
            }
            '+' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '+' {
                        tokens.push(Token::new(TokenKind::Inc, "++".to_string(), line, col));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Plus, "+".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Plus, "+".to_string(), line, col));
                }
            }
            '-' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '-' {
                        tokens.push(Token::new(TokenKind::Dec, "--".to_string(), line, col));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Minus, "-".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Minus, "-".to_string(), line, col));
                }
            }
            '*' => {
                tokens.push(Token::new(TokenKind::Star, "*".to_string(), line, col));
                chars.next();
            }
            '/' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '/' {
                        while let Some(&c) = chars.peek() {
                            if c == '\n' {
                                line += 1;
                                break;
                            }
                            chars.next();
                        }
                    } else if c == '*' {
                        // Multi-line comment
                        let mut last_char: char = '/';
                        while let Some(&c) = chars.peek() {
                            if c == '/' && last_char == '*' {
                                break;
                            } else if c == '\n' {
                                line += 1;
                            }
                            last_char = c;
                            chars.next();
                        }
                    } else {
                        tokens.push(Token::new(TokenKind::Slash, "/".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Slash, "/".to_string(), line, col));
                }
                chars.next();
            }
            '%' => {
                tokens.push(Token::new(TokenKind::Mod, "%".to_string(), line, col));
                chars.next();
            }
            '(' => {
                tokens.push(Token::new(TokenKind::LParen, "(".to_string(), line, col));
                chars.next();
            }
            ')' => {
                tokens.push(Token::new(TokenKind::RParen, ")".to_string(), line, col));
                chars.next();
            }
            '{' => {
                tokens.push(Token::new(TokenKind::LBrace, "{".to_string(), line, col));
                chars.next();
            }
            '}' => {
                tokens.push(Token::new(TokenKind::RBrace, "}".to_string(), line, col));
                chars.next();
            }
            '[' => {
                tokens.push(Token::new(TokenKind::LBrack, "[".to_string(), line, col));
                chars.next();
            }
            ']' => {
                tokens.push(Token::new(TokenKind::RBrack, "]".to_string(), line, col));
                chars.next();
            }
            '\n' => {
                line += 1;
                col = 0;
                chars.next();
            }
            ' ' | '\t' | '\r' | '\x0c' => {
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::Equal, "==".to_string(), line, col));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Assign, "=".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Assign, "=".to_string(), line, col));
                }
            }
            '<' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(
                            TokenKind::LessEqual,
                            "<=".to_string(),
                            line,
                            col,
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Less, "<".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Less, "<".to_string(), line, col));
                }
            }
            '>' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(
                            TokenKind::GreaterEqual,
                            ">=".to_string(),
                            line,
                            col,
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Greater, ">".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Greater, ">".to_string(), line, col));
                }
            }
            ';' => {
                tokens.push(Token::new(TokenKind::SemiColon, ";".to_string(), line, col));
                chars.next();
            }
            ':' => {
                tokens.push(Token::new(TokenKind::Colon, ":".to_string(), line, col));
                chars.next();
            }
            ',' => {
                tokens.push(Token::new(TokenKind::Comma, ",".to_string(), line, col));
                chars.next();
            }
            '!' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::NotEqual, "!=".to_string(), line, col));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Bang, "!".to_string(), line, col));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Bang, "!".to_string(), line, col));
                }
            }
            '"' | '\'' => {
                let chr = c;
                let mut value = String::new();
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == chr {
                        break;
                    }
                    if c == '\\' {
                        chars.next();
                        let n_0 = chars.peek();
                        if n_0.is_none() {
                            return error("Invalid escape character", line, col);
                        }
                        let n = n_0.unwrap();
                        let k = match n {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            _ => {
                                return error(
                                    format!("Invalid escape character \\{}", n).as_str(),
                                    line,
                                    col,
                                )
                            }
                        };
                        value.push(k);
                        chars.next();
                        continue;
                    }
                    value.push(c);
                    chars.next();
                }
                chars.next();
                tokens.push(Token::new(TokenKind::String, value, line, col));
            }
            '|' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '|' {
                        tokens.push(Token::new(TokenKind::Or, "||".to_string(), line, col));
                        chars.next();
                    } else {
                        return error(format!("Unknown character: '|{}'", c).as_str(), line, col);
                    }
                } else {
                    return error("Unknown character: '|'", line, col);
                }
            }
            '&' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '&' {
                        tokens.push(Token::new(TokenKind::And, "&&".to_string(), line, col));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::BitAnd, "&".to_string(), line, col));
                        chars.next();
                    }
                } else {
                    tokens.push(Token::new(TokenKind::BitAnd, "&".to_string(), line, col));
                    chars.next();
                }
            }
            '^' => {
                chars.next();
                tokens.push(Token::new(TokenKind::BitXor, "^".to_string(), line, col))
            }
            '.' => {
                chars.next();
                tokens.push(Token::new(TokenKind::Dot, ".".to_string(), line, col));
            }

            _ => {
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if is_identifier_char(c) {
                        value.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !value.is_empty() {
                    tokens.push(get_identifier(value, line, col));
                } else {
                    let c = chars.next().unwrap();
                    return error(format!("Unknown character: '{}'", c).as_str(), line, col);
                }
            }
        }
        col += 1;
    }

    tokens.push(Token::new(TokenKind::EOF, "".to_string(), line, col));
    Ok(tokens)
}
