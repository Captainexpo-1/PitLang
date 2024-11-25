use core::fmt;
use std::fmt::Debug;

use crate::virtual_machine::value::Value;

pub fn dump_bytecode(bytecode: &Bytecode) -> String {
    let mut output = String::new();
    output.push_str("Constants:\n");
    for (i, op) in bytecode.constants.iter().enumerate() {
        output.push_str(&format!("{:04}: {:?}\n", i, op));
    }
    output.push_str("\n--------------------------------\n");
    for (i, op) in bytecode.code.iter().enumerate() {
        output.push_str(&format!("{:04}: {:?}\n", i, op));
    }
    output
}

#[derive(Debug, Clone)]
pub enum OpCode {
    // Constants and stack manipulation
    CONST(u16), // Push constant at index to stack
    POP,        // Pop value from stack

    // Arithmetic operations
    ADD, // Add top two values on stack
    SUB, // Subtract top two values on stack
    MUL, // Multiply top two values on stack
    DIV, // Divide top two values on stack
    MOD, // Modulus of top two values on stack
    NEG, // Negate top value on stack

    // Control flow
    HALT,                 // Halt execution
    JUMP_IF_FALSE(usize), // Jump to address if top of stack is false
    JUMP(usize),          // Unconditional jump to address
    CALL { addr: usize, args: u16 },

    // Comparison operations
    EQ,  // Equal
    NEQ, // Not equal
    LT,  // Less than
    LTE, // Less than or equal to
    GT,  // Greater than
    GTE, // Greater than or equal to

    // For variables
    LOAD_GLOBAL(u16),  // Load global variable at index
    STORE_GLOBAL(u16), // Store top of stack in global variable at index
    LOAD_LOCAL(u16),   // Load local variable at index
    STORE_LOCAL(u16),  // Store top of stack in local variable at index

    // Function operations
    RETURN, // Return from function
}

#[derive(Default, Debug, Clone)]
pub struct Bytecode {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> Result<u16, String> {
        if self.constants.contains(&value) {
            let index = self.constants.iter().position(|x| *x == value).unwrap();
            return Ok(index as u16);
        }
        if self.constants.len() >= u16::MAX as usize {
            return Err("Too many constants".to_string());
        }
        let index = self.constants.len();
        self.constants.push(value);
        Ok(index as u16)
    }

    pub fn push_const(&mut self, value: Value) -> Result<u16, String> {
        let index = self.constants.len();
        self.constants.push(value);
        Ok(index as u16)
    }

    pub fn push_op(&mut self, op: OpCode) {
        self.code.push(op);
    }
}
