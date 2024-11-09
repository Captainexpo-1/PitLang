use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ASTNode;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Return(Box<Value>),
    Array(Rc<RefCell<Vec<Value>>>),
    Function_dep(Vec<String>, ASTNode),
    Function(Rc<Function>),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Method {
        receiver: Box<Value>,
        method_name: String,
    },
    Null,
    Unit,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            _ => true,
        }
    }
    pub fn print(&self) {
        match self {
            Value::Number(n) => print!("{}", n),
            Value::Boolean(b) => print!("{}", b),
            Value::String(s) => print!("{}", s),
            Value::Null => print!("null"),
            Value::Array(values) => {
                print!("[");
                for (i, val) in values.borrow().iter().enumerate() {
                    val.print();
                    if i < values.borrow().len() - 1 {
                        print!(", ");
                    }
                }
                print!("]");
            }
            Value::Object(_) => print!("Object"),
            Value::Function(_) => print!("Function"),
            Value::Function_dep(_, _) => print!("Function"),
            Value::Method {
                receiver,
                method_name,
            } => {
                print!("Method: {:?}.{}", receiver, method_name)
            }
            Value::Unit => (),
            _ => print!("Unsupported value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub parameters: Vec<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Stack manipulation
    LoadConst(usize), // Push constant at constants[usize] onto the stack
    LoadNull,         // Push Null onto the stack
    Pop,              // Pop the top value from the stack

    // Variables
    LoadGlobal(String),  // Load global variable onto the stack
    StoreGlobal(String), // Store top of stack into global variable

    // Arithmetic operations
    Add, // Add top two values on the stack
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison operations
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Control flow
    Jump(usize),        // Jump to instruction index
    JumpIfFalse(usize), // Jump if top of stack is false
    JumpIfTrue(usize),  // Jump if top of stack is true

    // Functions
    Call(usize), // Call function with given number of arguments
    Return,      // Return from function

    // Miscellaneous
    Print, // Print the top value on the stack
}
