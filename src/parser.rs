use crate::ast::ASTNode;
use crate::tokenizer::{Token, TokenKind};

pub fn parse(tokens: &[Token]) -> ASTNode {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse_program(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        while self.tokens[self.current].kind != TokenKind::EOF {
            statements.push(self.parse_statement());
        }
        ASTNode::Program(statements)
    }

    fn parse_statement(&mut self) -> ASTNode {
        let token: Token = self.tokens[self.current].clone();
        match token.kind {
            TokenKind::Let => self.parse_variable_declaration(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::Function => self.parse_function_declaration(true),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::SemiColon => {
                self.advance();
                self.parse_statement()
            }
            _ => {
                let expr = self.parse_expression(0);
                if self.tokens[self.current].kind == TokenKind::SemiColon {
                    self.expect(TokenKind::SemiColon);
                    ASTNode::Expression(Box::new(expr))
                } else {
                    expr
                }
            }
        }
    }

    fn parse_while_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::While);
        let condition = self.parse_expression(0);
        let body = Box::new(self.parse_block());
        ASTNode::WhileStatement {
            condition: Box::new(condition),
            body,
        }
    }

    fn parse_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();
        self.expect(TokenKind::LParen);
        if self.tokens[self.current].kind != TokenKind::RParen {
            loop {
                parameters.push(self.advance().value.clone());
                if self.tokens[self.current].kind == TokenKind::RParen {
                    break;
                }
                self.expect(TokenKind::Comma);
            }
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
        while self.tokens[self.current].kind != TokenKind::RBrace {
            statements.push(self.parse_statement());
        }
        self.expect(TokenKind::RBrace);
        ASTNode::Block(statements)
    }

    fn parse_if_statement(&mut self) -> ASTNode {
        self.expect(TokenKind::If);
        let condition = self.parse_expression(0);
        let consequence = self.parse_statement();
        let alternative = if self.tokens[self.current].kind == TokenKind::Else {
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
        let mut left = self.parse_nud();

        while self.current < self.tokens.len() && precedence < self.get_precedence() {
            left = self.parse_led(left);
        }

        left
    }

    fn parse_led(&mut self, left: ASTNode) -> ASTNode {
        let token = self.advance().clone();
        let precedence = self.get_precedence();

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
            TokenKind::Number => ASTNode::NumberLiteral(token.value.parse().unwrap()),
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
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn get_precedence(&self) -> u8 {
        match self.tokens[self.current].kind {
            TokenKind::Assign => 1,
            TokenKind::Equal | TokenKind::NotEqual => 2,
            TokenKind::Greater | TokenKind::Less => 2,
            TokenKind::GreaterEqual | TokenKind::LessEqual => 2,
            TokenKind::Plus | TokenKind::Minus => 2,
            TokenKind::Star | TokenKind::Slash | TokenKind::Mod => 3,
            TokenKind::LParen => 4, // For function calls
            TokenKind::Dot => 5,    // For member access
            _ => 0,
        }
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.tokens[self.current].kind != kind {
            panic!(
                "Expected token: {:?} at token #{:?}",
                kind, self.tokens[self.current]
            );
        }
        self.advance();
    }
}
