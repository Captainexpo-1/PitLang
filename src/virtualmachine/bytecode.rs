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
    // Stack Manipulation
    PushConst(usize), // Push a constant from the constant pool onto the stack
    Pop,              // Pop the top value off the stack
    Dup,              // Duplicate the top value on the stack

    // Arithmetic Operations
    Add, // Add top two values on the stack (supports numbers and strings)
    Sub, // Subtract top two values (numbers only)
    Mul, // Multiply top two values (numbers only)
    Div, // Divide top two values (numbers only)
    Mod,
    Negate, // Negate the top value (unary minus for numbers)

    // Comparison Operations
    Eq, // Check equality of top two values
    Ne, // Check inequality of top two values
    Gt, // Greater than
    Ge, // Greater than or equal to
    Lt, // Less than
    Le, // Less than or equal to

    // Logical Operations
    Not, // Logical NOT for booleans

    // Variable Operations
    LoadLocal(usize),   // Load a local variable onto the stack
    StoreLocal(usize),  // Store the top value in a local variable
    LoadGlobal(usize),  // Load a global variable onto the stack
    StoreGlobal(usize), // Store the top value in a global variable

    // Control Flow
    Jmp(usize),  // Unconditional jump to address
    Jit(usize),  // Jump if top of stack is true
    Jif(usize),  // Jump if top of stack is false
    Call(usize), // Call function at address
    Return,      // Return from function

    // Type Inspection
    TypeOf,      // Push the type of the top value as a string (e.g., "number", "string")
    IsNull,      // Check if top of stack is null
    IsUndefined, // Check if top of stack is undefined

    // Object Manipulation
    GetProperty(String), // Get a property from an Object (top of stack is the object)
    SetProperty(String), // Set a property on an Object (top is value, second is object)

    // Array Manipulation
    ArrayPush,       // Push a value to an Array (top of stack is value, below is array)
    ArrayPop,        // Pop a value from an Array (top of stack is the array)
    ArrayGet(usize), // Get an element at index in an Array
    ArraySet(usize), // Set an element at index in an Array

    DEBUG_LABEL(String), // Debug label for debugging purposes

    Swap,

    // Program Termination
    Halt, // Stop execution
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
