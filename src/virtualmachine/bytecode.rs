use crate::virtualmachine::value::Value;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushConst(u16), // push const to stack from constant pool

    Pop,  // pop value from stack
    Dup,  // duplicate top value on stack
    Swap, // swap top two values on stack

    // Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Jmp(u16), // jump to instruction
    Jit(u16), // jump if top of stack is true
    Jif(u16), // jump if top of stack is false

    // Comparison operations
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,

    Return, // return from function and jump to return address on stack
    Halt,   // halt execution
}

pub fn dump_bytecode(code: &Bytecode, path: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    for (i, instr) in code.instructions.iter().enumerate() {
        let t = format!("{:?}", instr);
        writeln!(file, "{:04} {}", i, t).unwrap();
    }
    write!(file, "\n\nConstants:\n").unwrap();
    for (i, constant) in code.constants.iter().enumerate() {
        let t = format!("{:?}", constant);
        writeln!(file, "{:04} {}", i, t).unwrap();
    }
    writeln!(file).unwrap();
}
