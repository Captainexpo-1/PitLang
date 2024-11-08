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
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token { kind, value }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn get_identifier(id: String) -> Token {
    match id.as_str() {
        "let" => Token::new(TokenKind::Let, id),
        "fn" => Token::new(TokenKind::Function, id),
        "if" => Token::new(TokenKind::If, id),
        "else" => Token::new(TokenKind::Else, id),
        "return" => Token::new(TokenKind::Return, id),
        _ => Token::new(TokenKind::Identifier, id),
    }
}

pub fn tokenize(text: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        value.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenKind::Number, value));
            }
            '+' => {
                tokens.push(Token::new(TokenKind::Plus, "+".to_string()));
                chars.next();
            }
            '-' => {
                tokens.push(Token::new(TokenKind::Minus, "-".to_string()));
                chars.next();
            }
            '*' => {
                tokens.push(Token::new(TokenKind::Star, "*".to_string()));
                chars.next();
            }
            '/' => {
                tokens.push(Token::new(TokenKind::Slash, "/".to_string()));
                chars.next();
            }
            '(' => {
                tokens.push(Token::new(TokenKind::LParen, "(".to_string()));
                chars.next();
            }
            ')' => {
                tokens.push(Token::new(TokenKind::RParen, ")".to_string()));
                chars.next();
            }
            '{' => {
                tokens.push(Token::new(TokenKind::LBrace, "{".to_string()));
                chars.next();
            }
            '}' => {
                tokens.push(Token::new(TokenKind::RBrace, "}".to_string()));
                chars.next();
            }
            ' ' | '\t' | '\r' | '\n' | '\x0c' => {
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::Equal, "==".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Assign, "=".to_string()));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Assign, "=".to_string()));
                }
            }
            '<' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::LessEqual, "<=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Assign, "<=".to_string()));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Assign, "<=".to_string()));
                }
            }
            '>' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::GreaterEqual, ">=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Greater, ">".to_string()));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Greater, ">".to_string()));
                }
            }
            ';' => {
                tokens.push(Token::new(TokenKind::SemiColon, ";".to_string()));
                chars.next();
            }
            ':' => {
                tokens.push(Token::new(TokenKind::Colon, ":".to_string()));
                chars.next();
            }
            ',' => {
                tokens.push(Token::new(TokenKind::Comma, ",".to_string()));
                chars.next();
            }
            '!' => {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c == '=' {
                        tokens.push(Token::new(TokenKind::NotEqual, "!=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Bang, "!".to_string()));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Bang, "!".to_string()));
                }
            }
            '"' => {
                let mut value = String::new();
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        break;
                    }
                    value.push(c);
                    chars.next();
                }
                chars.next();
                tokens.push(Token::new(TokenKind::String, value));
            }
            '.' => {
                chars.next();
                tokens.push(Token::new(TokenKind::Dot, ".".to_string()));
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
                    tokens.push(get_identifier(value));
                } else {
                    let c = chars.next().unwrap();
                    panic!("Unknown character: '{}'", c);
                }
            }
        }
    }

    tokens.push(Token::new(TokenKind::EOF, "".to_string()));
    tokens
}
