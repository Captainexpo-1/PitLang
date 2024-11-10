use crate::common::Value;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum Bytecode {
    // Data manipulation
    LoadConst(usize), // Load a constant by index onto the stack
    LoadVar(String),  // Load a variable by name
    StoreVar(String), // Store the top of the stack into a variable

    // Math operations
    Add, // Add two numbers on the stack
    Sub, // Subtract two numbers on the stack
    Mul, // Multiply two numbers on the stack
    Div, // Divide two numbers on the stack
    Mod, // Modulo two numbers on the stack

    // Logic operations
    Eq,      // Check if two values on the stack are equal
    NotEq,   // Check if two values on the stack are not equal
    Lt,      // Check if the second stack value is less than the top
    Gt,      // Check if the second stack value is greater than the top
    LtEqual, // Check if the second stack value is less than or equal to the top
    GtEqual, // Check if the second stack value is greater than or equal to the top

    // Control flow
    Jump(usize),        // Jump to an absolute position in the bytecode
    JumpIfFalse(usize), // Jump if the top stack value is false

    // Functions
    Call(usize), // Call a function with a certain number of arguments
    Return,      // Return from a function

    // Object and list manipulation
    GetProp(String),   // Get a property from an object
    SetProp(String),   // Set a property on an object
    BuildArray(usize), // Build an array from the top n values on the stack

    // Misc
    Duplicate, // Duplicate the top value on the stack
}

pub fn dump_bytecode(bytecode: &[Bytecode], constants: &[Value], path: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    for (i, instr) in bytecode.iter().enumerate() {
        let t = format!("{:?}", instr);
        writeln!(file, "{:04} {}", i, t).unwrap();
    }
    write!(file, "\n\nConstants:\n").unwrap();
    for (i, constant) in constants.iter().enumerate() {
        let t = format!("{:?}", constant);
        writeln!(file, "{:04} {}", i, t).unwrap();
    }
    writeln!(file).unwrap();
}
