#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack Manipulation
    PushConst(usize), // Push a constant value onto the stack (index into constants pool)
    Pop,              // Pop the top value off the stack
    Dup,              // Duplicate the top value on the stack

    // Arithmetic Operations
    Add,      // Add two numbers
    Subtract, // Subtract two numbers
    Multiply, // Multiply two numbers
    Divide,   // Divide two numbers
    Modulo,   // Modulo operation

    // Logical Operations
    Equal,              // Check equality
    NotEqual,           // Check inequality
    GreaterThan,        // Check if greater than
    GreaterThanOrEqual, // Check if greater than or equal
    LessThan,           // Check if less than
    LessThanOrEqual,    // Check if less than or equal
    LogicalAnd,         // Logical AND
    LogicalOr,          // Logical OR
    LogicalNot,         // Logical NOT

    // Variable Operations
    LoadGlobal(String),  // Load a global variable onto the stack
    StoreGlobal(String), // Store the top stack value into a global variable
    LoadLocal(usize),    // Load a local variable onto the stack (by index)
    StoreLocal(usize),   // Store the top stack value into a local variable (by index)

    // Control Flow
    Jump(usize),        // Unconditional jump to an instruction index
    JumpIfTrue(usize),  // Jump if the top stack value is true
    JumpIfFalse(usize), // Jump if the top stack value is false
    Call(usize, usize), // Call a function (address, number of arguments)
    Return,             // Return from a function

    // Miscellaneous
    Print, // Print the top value on the stack
    Nop,   // No operation
    Halt,  // Stop execution
}
