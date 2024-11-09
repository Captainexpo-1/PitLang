use crate::ast::ASTNode;
use crate::common::Value;
use crate::tokenizer::TokenKind;
use crate::virtualmachine::bytecode::Bytecode;

#[derive(Clone, PartialEq, Debug)]
pub struct CodeGenerator {
    bytecode: Vec<Bytecode>,
    constants: Vec<Value>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            bytecode: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn generate_bytecode(&mut self, ast: &ASTNode) -> (Vec<Bytecode>, Vec<Value>) {
        match ast {
            ASTNode::Program(nodes) => {
                for node in nodes {
                    self.generate_bytecode_inner(node);
                }
            }
            _ => self.generate_bytecode_inner(ast),
        }
        (self.bytecode.clone(), self.constants.clone())
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn generate_bytecode_inner(&mut self, node: &ASTNode) {
        match node {
            ASTNode::NumberLiteral(n) => {
                let index = self.add_constant(Value::Number(*n));
                self.bytecode.push(Bytecode::LoadConst(index));
            }
            ASTNode::StringLiteral(s) => {
                let index = self.add_constant(Value::String(s.clone()));
                self.bytecode.push(Bytecode::LoadConst(index));
            }
            ASTNode::BooleanLiteral(b) => {
                let index = self.add_constant(Value::Boolean(*b));
                self.bytecode.push(Bytecode::LoadConst(index));
            }
            ASTNode::NullLiteral => {
                let index = self.add_constant(Value::Null);
                self.bytecode.push(Bytecode::LoadConst(index));
            }
            ASTNode::Variable(name) => {
                self.bytecode.push(Bytecode::LoadVar(name.clone()));
            }
            ASTNode::BinaryOp { left, op, right } => match op {
                TokenKind::Assign => {
                    if let ASTNode::Variable(var_name) = &**left {
                        self.generate_bytecode_inner(right);
                        self.bytecode.push(Bytecode::StoreVar(var_name.clone()));
                    } else {
                        unimplemented!("Assignment to non-variable is not supported.");
                    }
                }
                _ => {
                    self.generate_bytecode_inner(left);
                    self.generate_bytecode_inner(right);
                    match op {
                        TokenKind::Plus => self.bytecode.push(Bytecode::Add),
                        TokenKind::Minus => self.bytecode.push(Bytecode::Sub),
                        TokenKind::Star => self.bytecode.push(Bytecode::Mul),
                        TokenKind::Slash => self.bytecode.push(Bytecode::Div),
                        TokenKind::Equal => self.bytecode.push(Bytecode::Eq),
                        TokenKind::NotEqual => self.bytecode.push(Bytecode::NotEq),
                        TokenKind::Greater => self.bytecode.push(Bytecode::Gt),
                        TokenKind::GreaterEqual => self.bytecode.push(Bytecode::GtEqual),
                        TokenKind::Less => self.bytecode.push(Bytecode::Lt),
                        TokenKind::LessEqual => self.bytecode.push(Bytecode::LtEqual),
                        _ => unimplemented!(),
                    }
                }
            },
            ASTNode::UnaryOp { op, operand } => {
                self.generate_bytecode_inner(operand);
                match op {
                    TokenKind::Minus => {
                        let index = self.add_constant(Value::Number(-1.0));
                        self.bytecode.push(Bytecode::LoadConst(index));
                        self.bytecode.push(Bytecode::Mul);
                    }
                    //TokenKind::Bang => self.bytecode.push(Bytecode::Not),
                    _ => unimplemented!(),
                }
            }
            ASTNode::VariableDeclaration { name, value } => {
                self.generate_bytecode_inner(value);
                self.bytecode.push(Bytecode::StoreVar(name.clone()));
            }
            ASTNode::Block(nodes) => {
                for node in nodes {
                    self.generate_bytecode_inner(node);
                }
            }
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            } => {
                self.generate_bytecode_inner(condition);
                let jump_if_false_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::JumpIfFalse(0)); // Placeholder
                self.generate_bytecode_inner(consequence);
                let jump_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::Jump(0)); // Placeholder
                let else_pos = self.bytecode.len();
                self.bytecode[jump_if_false_pos] = Bytecode::JumpIfFalse(else_pos);
                if let Some(alt) = alternative {
                    self.generate_bytecode_inner(alt);
                }
                let end_pos = self.bytecode.len();
                self.bytecode[jump_pos] = Bytecode::Jump(end_pos);
            }
            ASTNode::WhileStatement { condition, body } => {
                let start_pos = self.bytecode.len();
                self.generate_bytecode_inner(condition);
                let jump_if_false_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::JumpIfFalse(0)); // Placeholder
                self.generate_bytecode_inner(body);
                self.bytecode.push(Bytecode::Jump(start_pos));
                let end_pos = self.bytecode.len();
                self.bytecode[jump_if_false_pos] = Bytecode::JumpIfFalse(end_pos);
            }
            ASTNode::Expression(expr) => {
                self.generate_bytecode_inner(expr);
            }
            _ => unimplemented!(),
        }
    }
}
