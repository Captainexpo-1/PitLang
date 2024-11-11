use crate::virtualmachine::bytecode::Instruction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub is_main: bool,         // Is this the main frame?
    pub return_address: usize, // Address to jump back to after function call
    pub locals: Vec<Value>,    // Local variables for this frame
}

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
    Function(FunctionMeta),
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
                s.push(']');
                s
            }
            Value::Object(_) => "Object".to_string(),
            Value::Function(_) => "Function".to_string(),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Value::Number(_) => "number".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Function(_) => "function".to_string(),
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
pub struct FunctionMeta {
    pub address: usize,
    pub arity: usize,
}
