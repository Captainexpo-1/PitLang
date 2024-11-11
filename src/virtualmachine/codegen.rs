use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::treewalk::value::Value;
use crate::virtualmachine::bytecode::Instruction;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct CodeGenerator {
    bytecode: Vec<Instruction>,
    constants: Vec<Value>,
    variable_indices: HashMap<String, u16>,
    next_variable_index: u16,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            bytecode: Vec::new(),
            constants: Vec::new(),
            variable_indices: HashMap::new(),
            next_variable_index: 0,
        }
    }

    pub fn generate_bytecode(
        &mut self,
        ast: &ASTNode,
        parameters: Option<&Vec<String>>,
    ) -> (Vec<Instruction>, Vec<Value>) {
        // Include parameters in variable_indices
        return (vec![], vec![]);
    }
}
