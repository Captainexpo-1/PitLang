use crate::ast::ASTNode;
use crate::common::ParserError;
use crate::tokenizer::{Token, TokenKind};

pub fn parse(tokens: &[Token]) -> Result<ASTNode, Vec<ParserError>> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    fn synchronize_tokens(&mut self) {
        while self.current < self.tokens.len() {
            if self.tokens[self.current].kind == TokenKind::SemiColon {
                self.advance();
                return;
            }
            self.current += 1;
        }
    }

    fn error(&mut self, message: &str, token: &Token) {
        self.errors
            .push(ParserError::new(message, token.line, token.column));
        self.synchronize_tokens();
    }

    fn parse_program(&mut self) -> Result<ASTNode, Vec<ParserError>> {
        let mut statements = Vec::new();
        while self.current < self.tokens.len() && self.tokens[self.current].kind != TokenKind::EOF {
            statements.push(self.parse_statement());
        }
        if self.errors.is_empty() {
            Ok(ASTNode::Program(statements))
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_statement(&mut self) -> ASTNode {
        if self.current >= self.tokens.len() {
            self.error(
                "Unexpected end of input in statement",
                &self.tokens[self.tokens.len() - 1],
            );
            return ASTNode::NullLiteral;
        }

        let token = self.tokens[self.current].clone();
        match token.kind {
            TokenKind::Let => self.parse_variable_declaration(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::Function => self.parse_function_declaration(true),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::SemiColon => {
                self.advance();
                self.parse_statement()
            }
            _ => {
                let expr = self.parse_expression(0);
                if self.current < self.tokens.len()
                    && self.tokens[self.current].kind == TokenKind::SemiColon
                {
                    self.expect(TokenKind::SemiColon);
                }
                expr
            }
        }
    }

    fn parse_for_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::For);
        let pre = self.parse_statement();
        let cond = self.parse_expression(0);
        let iter = self.parse_statement();
        let block = self.parse_block();
        ASTNode::ForStatement {
            start: Box::new(pre),
            condition: Box::new(cond),
            iter: Box::new(iter),
            body: Box::new(block),
        }
    }

    fn parse_while_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::While);
        let condition = self.parse_expression(0);
        let body = Box::new(self.parse_statement());
        ASTNode::WhileStatement {
            condition: Box::new(condition),
            body,
        }
    }

    fn parse_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();
        self.expect(TokenKind::LParen);
        while self.current < self.tokens.len()
            && self.tokens[self.current].kind != TokenKind::RParen
        {
            parameters.push(self.advance().value.clone());
            if self.current < self.tokens.len()
                && self.tokens[self.current].kind == TokenKind::RParen
            {
                break;
            }
            self.expect(TokenKind::Comma);
        }
        self.expect(TokenKind::RParen);
        parameters
    }

    fn parse_return_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::Return);
        if self.tokens[self.current].kind == TokenKind::SemiColon {
            self.expect(TokenKind::SemiColon);
            return ASTNode::ReturnStatement(Box::new(ASTNode::NullLiteral));
        }
        let returnee = self.parse_expression(0);
        self.expect(TokenKind::SemiColon);
        ASTNode::ReturnStatement(Box::new(returnee))
    }

    fn parse_function_declaration(&mut self, not_anonymous: bool) -> ASTNode {
        if not_anonymous {
            self.expect(TokenKind::Function);
        }
        let name = if not_anonymous {
            Some(self.advance().value.clone())
        } else {
            None
        };
        let parameters = self.parse_parameters();
        let body = Box::new(self.parse_block());
        ASTNode::FunctionDeclaration {
            name,
            parameters,
            body,
        }
    }
    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.expect(TokenKind::Let);
        let name = self.advance().value.clone();
        self.expect(TokenKind::Assign);
        let value = self.parse_expression(0);
        self.expect(TokenKind::SemiColon);
        ASTNode::VariableDeclaration {
            name,
            value: Box::new(value),
        }
    }

    fn parse_block(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        self.expect(TokenKind::LBrace);

        if self.current >= self.tokens.len() {
            self.error(
                "Unexpected end of input in block",
                &self.tokens[self.tokens.len() - 1],
            );
            return ASTNode::Block(statements);
        }

        while self.tokens[self.current].kind != TokenKind::RBrace {
            statements.push(self.parse_statement());

            if self.current >= self.tokens.len() {
                self.error(
                    "Unexpected end of input in block",
                    &self.tokens[self.tokens.len() - 1],
                );
                break;
            }
        }
        self.expect(TokenKind::RBrace);
        ASTNode::Block(statements)
    }

    fn parse_if_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::If);
        let condition = self.parse_expression(0);
        let consequence = self.parse_statement();
        let alternative = if self.current < self.tokens.len()
            && self.tokens[self.current].kind == TokenKind::Else
        {
            self.advance();
            Some(Box::new(self.parse_statement()))
        } else {
            None
        };
        ASTNode::IfStatement {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        }
    }

    fn parse_expression(&mut self, precedence: u8) -> ASTNode {
        if self.current >= self.tokens.len() {
            self.error(
                "Unexpected end of input in expression",
                &self.tokens[self.tokens.len() - 1],
            );
            return ASTNode::NullLiteral;
        }

        let mut left = self.parse_nud();

        while self.current < self.tokens.len()
            && precedence < self.get_operator_precedence(&self.tokens[self.current].kind)
        {
            left = self.parse_led(left);
        }

        left
    }

    fn parse_led(&mut self, left: ASTNode) -> ASTNode {
        let token = self.advance().clone();
        let precedence = self.get_operator_precedence(&token.kind);

        match token.kind {
            TokenKind::Assign => {
                let right = self.parse_expression(precedence);
                ASTNode::BinaryOp {
                    left: Box::new(left),
                    op: token.kind,
                    right: Box::new(right),
                }
            }
            TokenKind::LParen => {
                let arguments = self.parse_arguments();
                ASTNode::FunctionCall {
                    callee: Box::new(left), // Accept any ASTNode as callee
                    arguments,
                }
            }
            TokenKind::Dot => {
                let member = self.advance().value.clone();
                ASTNode::MemberAccess {
                    object: Box::new(left),
                    member,
                }
            }
            _ => {
                let right = self.parse_expression(precedence);
                ASTNode::BinaryOp {
                    left: Box::new(left),
                    op: token.kind,
                    right: Box::new(right),
                }
            }
        }
    }

    fn parse_arguments(&mut self) -> Vec<ASTNode> {
        let mut arguments = Vec::new();
        if self.tokens[self.current].kind != TokenKind::RParen {
            loop {
                arguments.push(self.parse_expression(0));
                if self.current >= self.tokens.len() {
                    break;
                }
                if self.tokens[self.current].kind == TokenKind::RParen {
                    break;
                }
                self.expect(TokenKind::Comma);
            }
        }
        self.expect(TokenKind::RParen);
        arguments
    }

    fn parse_nud(&mut self) -> ASTNode {
        let token = self.advance();
        match token.kind {
            TokenKind::Number => {
                let num = token.value.parse();
                if let Ok(n) = num {
                    ASTNode::NumberLiteral(n)
                } else {
                    let t = &token.clone();
                    self.error("Failed to parse number", t);
                    ASTNode::NullLiteral
                }
            }
            TokenKind::String => ASTNode::StringLiteral(token.value.clone()),
            TokenKind::Identifier => ASTNode::Variable(token.value.clone()),
            TokenKind::Function => self.parse_function_declaration(false),
            TokenKind::True => ASTNode::BooleanLiteral(true),
            TokenKind::False => ASTNode::BooleanLiteral(false),
            TokenKind::Null => ASTNode::NullLiteral,
            TokenKind::Dot => {
                let member = self.advance().value.clone();
                ASTNode::MemberAccess {
                    object: Box::new(self.parse_nud()),
                    member,
                }
            }
            TokenKind::LParen => {
                let expr = self.parse_expression(0);
                self.expect(TokenKind::RParen);
                expr
            }
            TokenKind::LBrace => {
                let mut properties: Vec<(String, ASTNode)> = Vec::new();

                while self.tokens[self.current].kind != TokenKind::RBrace {
                    let key = self.advance().value.clone();
                    self.expect(TokenKind::Colon);
                    let value = self.parse_expression(0);
                    properties.push((key, value));
                    if self.tokens[self.current].kind == TokenKind::Comma {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RBrace);
                ASTNode::ObjectLiteral(properties)
            }
            TokenKind::LBrack => {
                let mut elements: Vec<ASTNode> = Vec::new();

                while self.tokens[self.current].kind != TokenKind::RBrack {
                    elements.push(self.parse_expression(0));
                    if self.tokens[self.current].kind == TokenKind::Comma {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RBrack);
                ASTNode::ArrayLiteral(elements)
            }
            TokenKind::Minus => ASTNode::UnaryOp {
                op: token.kind,
                operand: Box::new(self.parse_expression(3)),
            },
            TokenKind::Bang => ASTNode::UnaryOp {
                op: token.kind,
                operand: Box::new(self.parse_expression(3)),
            },
            TokenKind::Inc => ASTNode::UnaryOp {
                op: token.kind,
                operand: Box::new(self.parse_expression(3)),
            },
            TokenKind::Dec => ASTNode::UnaryOp {
                op: token.kind,
                operand: Box::new(self.parse_expression(3)),
            },
            _ => {
                let kind = token.kind;
                let token = token.clone();
                self.error(&format!("Unexpected token: {:?}", kind), &token);
                ASTNode::NullLiteral
            }
        }
    }

    fn get_operator_precedence(&self, kind: &TokenKind) -> u8 {
        match kind {
            TokenKind::Assign => 1,
            TokenKind::Inc | TokenKind::Dec => 2,
            TokenKind::Or => 3,
            TokenKind::And => 4,
            TokenKind::BitAnd => 5,
            TokenKind::BitXor => 6,
            TokenKind::BitOr => 7,
            TokenKind::Equal | TokenKind::NotEqual => 8,
            TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => 9,
            TokenKind::Plus | TokenKind::Minus => 10,
            TokenKind::Star | TokenKind::Mod | TokenKind::Slash => 11,
            TokenKind::LParen => 12,
            TokenKind::Dot => 13,
            _ => 0,
        }
    }

    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            token
        } else {
            // Return a dummy EOF token or handle the error
            self.error(
                "Unexpected end of input",
                &self.tokens[self.tokens.len() - 1],
            );
            &self.tokens[self.tokens.len() - 1] // Return the last token to avoid panic
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.current >= self.tokens.len() || self.tokens[self.current].kind != kind {
            let token = if self.current < self.tokens.len() {
                &self.tokens[self.current]
            } else {
                &self.tokens[self.tokens.len() - 1]
            };
            self.error(&format!("Expected token: {:?}", kind), token);
        } else {
            self.advance();
        }
    }
}
