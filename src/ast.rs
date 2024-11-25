use crate::tokenizer::TokenKind;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ASTNode {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    Expression(Box<ASTNode>),
    Variable(String),
    Program(Vec<ASTNode>),
    Block(Vec<ASTNode>),
    ObjectLiteral(Vec<(String, ASTNode)>),
    ArrayLiteral(Vec<ASTNode>),
    BinaryOp {
        left: Box<ASTNode>,
        op: TokenKind,
        right: Box<ASTNode>,
    },
    UnaryOp {
        op: TokenKind,
        operand: Box<ASTNode>,
    },
    VariableDeclaration {
        name: String,
        value: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        consequence: Box<ASTNode>,
        alternative: Option<Box<ASTNode>>,
    },
    FunctionCall {
        callee: Box<ASTNode>,
        arguments: Vec<ASTNode>,
    },
    FunctionDeclaration {
        name: Option<String>,
        parameters: Vec<String>,
        body: Box<ASTNode>,
    },
    ReturnStatement(Box<ASTNode>),
    WhileStatement {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    MemberAccess {
        object: Box<ASTNode>,
        member: String,
    },
}
