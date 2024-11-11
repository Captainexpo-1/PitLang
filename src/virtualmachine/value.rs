use crate::virtualmachine::bytecode::Instruction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type StdMethod = fn(&Value, Vec<Value>) -> Value; // Takes a receiver and arguments, returns a value

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Object(Object),
    Array(Vec<Value>),
    Function(usize), // Entry point in the instruction array
    Null,
    Undefined,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            _ => true,
        }
    }
    pub fn print(&self) {
        print!("{}", self.to_string());
    }
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Array(values) => {
                let mut s = "[".to_string();
                for (i, val) in values.iter().enumerate() {
                    s.push_str(&val.to_string());
                    if i < values.len() - 1 {
                        s.push_str(", ");
                    }
                }
                s.push_str("]");
                s
            }
            Value::Object(_) => "Object".to_string(),
            Value::Function(_) => "Function".to_string(),
        }
    }

    pub fn less_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            _ => false,
        }
    }

    pub fn less_than_or_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a <= b,
            _ => false,
        }
    }

    pub fn greater_than(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a > b,
            (Value::String(a), Value::String(b)) => a > b,
            _ => false,
        }
    }

    pub fn greater_than_or_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a >= b,
            (Value::String(a), Value::String(b)) => a >= b,
            _ => false,
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
