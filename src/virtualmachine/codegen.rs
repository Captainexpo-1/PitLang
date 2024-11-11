use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::tokenizer::TokenKind;
use crate::virtualmachine::bytecode::{Bytecode, Instruction};
use crate::virtualmachine::value::{FunctionMeta, Value};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct CodeGenerator {
    bytecode: Vec<Instruction>,
    constants: Vec<Value>,
    variable_indices: HashMap<String, u16>,
    variables: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            bytecode: Vec::new(),
            constants: Vec::new(),
            variable_indices: HashMap::new(),
            variables: 0,
        }
    }

    pub fn generate_bytecode(&mut self, ast: &ASTNode) -> Bytecode {
        self.visit_node(ast);
        self.bytecode.push(Instruction::Halt);
        Bytecode {
            instructions: self.bytecode.clone(),
            constants: self.constants.clone(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        if self.constants.contains(&value) {
            return self.constants.iter().position(|x| *x == value).unwrap();
        }
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn visit_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(statements) => {
                for statement in statements {
                    self.visit_node(statement);
                }
            }
            ASTNode::NumberLiteral(n) => {
                let c = self.add_constant(Value::Number(*n));
                self.bytecode.push(Instruction::PushConst(c));
            }
            ASTNode::StringLiteral(s) => {
                let c = self.add_constant(Value::String(s.clone()));
                self.bytecode.push(Instruction::PushConst(c));
            }
            ASTNode::BooleanLiteral(b) => {
                let c = self.add_constant(Value::Bool(*b));
                self.bytecode.push(Instruction::PushConst(c));
            }
            ASTNode::NullLiteral => {
                let c = self.add_constant(Value::Null);
                self.bytecode.push(Instruction::PushConst(c));
            }
            ASTNode::BinaryOp { left, op, right } => {
                self.visit_node(right);
                self.visit_node(left);
                match op {
                    TokenKind::Plus => self.bytecode.push(Instruction::Add),
                    TokenKind::Minus => self.bytecode.push(Instruction::Sub),
                    TokenKind::Star => self.bytecode.push(Instruction::Mul),
                    TokenKind::Slash => self.bytecode.push(Instruction::Div),
                    TokenKind::Equal => self.bytecode.push(Instruction::Eq),
                    TokenKind::NotEqual => self.bytecode.push(Instruction::Ne),
                    TokenKind::Greater => self.bytecode.push(Instruction::Gt),
                    TokenKind::GreaterEqual => self.bytecode.push(Instruction::Ge),
                    TokenKind::Less => self.bytecode.push(Instruction::Lt),
                    TokenKind::LessEqual => self.bytecode.push(Instruction::Le),

                    _ => panic!("Unknown binary operator: {:?}", op),
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                self.visit_node(operand);
                match op {
                    TokenKind::Minus => self.bytecode.push(Instruction::Negate),
                    TokenKind::Bang => self.bytecode.push(Instruction::Not),
                    _ => panic!("Unknown unary operator: {:?}", op),
                }
            }
            ASTNode::VariableDeclaration { name, value } => {
                self.visit_node(value); // Evaluate the value
                let idx = self.variables;
                self.variable_indices.insert(name.clone(), idx as u16);
                self.variables += 1;
                self.bytecode.push(Instruction::StoreLocal(idx as usize)); // Store the result in a local variable slot
            }
            ASTNode::Variable(name) => {
                if let Some(&idx) = self.variable_indices.get(name) {
                    self.bytecode.push(Instruction::LoadLocal(idx as usize)); // Load the variable onto the stack
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }
            ASTNode::Block(statements) => {
                for statement in statements {
                    self.visit_node(statement);
                }
            }
            ASTNode::FunctionDeclaration {
                name,
                parameters,
                body,
            } => {
                self.visit_function(name.as_ref().unwrap(), parameters.clone(), body);
            }
            ASTNode::ReturnStatement(value) => {
                self.visit_node(value);
                self.bytecode.push(Instruction::Return);
            }
            _ => {
                unimplemented!("{:?}", node);
            }
        }
    }

    pub fn visit_function(&mut self, name: &str, params: Vec<String>, body: &ASTNode) {
        // Record the starting address of the function
        self.bytecode
            .push(Instruction::DEBUG_LABEL(name.to_string()));

        let function_address = self.bytecode.len();

        // Set up function metadata with the address and parameter count
        let function_meta = FunctionMeta {
            address: function_address,
            arity: params.len(),
        };

        // Add function metadata to constants
        let func_index = self.add_constant(Value::Function(function_meta));

        // Store function in variable index map, treating functions as variables
        self.variable_indices
            .insert(name.to_string(), func_index as u16);

        // Compile the function body
        for (i, param) in params.iter().enumerate() {
            self.variable_indices.insert(param.clone(), i as u16);
        }
        self.visit_node(body);

        if self.bytecode.last() != Some(&Instruction::Return) {
            let c = self.add_constant(Value::Null);
            self.bytecode.push(Instruction::PushConst(c));
            self.bytecode.push(Instruction::Return);
        }
    }
}
