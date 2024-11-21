use crate::virtual_machine::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    CONST(u16), // Push constant at index to stack,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    NEG,
    HALT,
    JUMP_IF_FALSE(usize),
    JUMP(usize),
    EQ,
    NEQ,
    LT,
    LTE,
    GT,
    GTE,
}

#[derive(Default, Debug)]
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
