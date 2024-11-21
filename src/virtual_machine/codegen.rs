use crate::ast::ASTNode;
use crate::tokenizer::TokenKind;
use crate::virtual_machine::bytecode::{Bytecode, OpCode};
use crate::virtual_machine::value::{Value, ValueType};

#[derive(Default)]
pub struct Codegen {
    pub bytecode: Bytecode,
}

impl Codegen {
    pub fn dump_bytecode(&self) -> String {
        let mut output = String::new();
        output.push_str("Constants:\n");
        for i in self.bytecode.constants.iter() {
            output.push_str(&format!("{:?}\n", i));
        }
        output.push_str("\n--------------------------------\n\n");
        for (i, op) in self.bytecode.code.iter().enumerate() {
            output.push_str(&format!("{:04}: {:?}\n", i, op));
        }
        output
    }
    pub fn new() -> Self {
        Self {
            bytecode: Bytecode::new(),
        }
    }
    pub fn generate_op(&mut self, op: &TokenKind) -> Result<OpCode, String> {
        match op {
            TokenKind::Plus => Ok(OpCode::ADD),
            TokenKind::Minus => Ok(OpCode::SUB),
            TokenKind::Star => Ok(OpCode::MUL),
            TokenKind::Slash => Ok(OpCode::DIV),
            TokenKind::Mod => Ok(OpCode::MOD),
            TokenKind::Greater => Ok(OpCode::GT),
            TokenKind::GreaterEqual => Ok(OpCode::GTE),
            TokenKind::Less => Ok(OpCode::LT),
            TokenKind::LessEqual => Ok(OpCode::LTE),
            TokenKind::Equal => Ok(OpCode::EQ),
            TokenKind::NotEqual => Ok(OpCode::NEQ),
            _ => panic!("Unknown operator: {:?}", op),
        }
    }
    pub fn generate(&mut self, node: &ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Program(statements) => {
                for statement in statements {
                    self.generate(statement)?;
                }
                Ok(())
            }
            ASTNode::Block(statements) => {
                for statement in statements {
                    self.generate(statement)?;
                }
                Ok(())
            }
            ASTNode::NumberLiteral(value) => {
                let const_index = self.bytecode.push_const(Value::new_float(*value))?;
                self.bytecode.code.push(OpCode::CONST(const_index));
                Ok(())
            }
            ASTNode::BinaryOp { op, left, right } => {
                self.generate(left)?;
                self.generate(right)?;
                let opcode = self.generate_op(op)?;
                self.bytecode.code.push(opcode);
                Ok(())
            }
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            } => {
                // Generate code for the condition
                self.generate(condition)?;

                // Placeholder for JUMP_IF_FALSE, will update later
                let jump_if_false_pos = self.bytecode.code.len();
                self.bytecode.code.push(OpCode::JUMP_IF_FALSE(0));

                // Generate code for the consequence (then-branch)
                self.generate(consequence)?;

                // Placeholder for JUMP, will update later (only if there's an else)
                let jump_pos = if alternative.is_some() {
                    let pos = self.bytecode.code.len();
                    self.bytecode.code.push(OpCode::JUMP(0));
                    Some(pos)
                } else {
                    None
                };

                // Update JUMP_IF_FALSE to point to the else branch or after if-statement
                let after_consequence = self.bytecode.code.len();
                self.bytecode.code[jump_if_false_pos] = OpCode::JUMP_IF_FALSE(after_consequence);

                if let Some(alt) = alternative {
                    // Generate code for the alternative (else-branch)
                    self.generate(alt)?;

                    // Update JUMP to point to the instruction after the else-branch
                    if let Some(jump_pos) = jump_pos {
                        let after_alternative = self.bytecode.code.len();
                        self.bytecode.code[jump_pos] = OpCode::JUMP(after_alternative);
                    } else {
                        return Err("Expected JUMP opcode position".to_string());
                    }
                }

                Ok(())
            }
            // Handle other AST node types...
            _ => Err(format!("Unknown node: {:?}", node)),
        }
    }
}
