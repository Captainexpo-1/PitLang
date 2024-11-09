use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::virtualmachine::bytecode::Bytecode;

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
    pub instructions: Vec<Bytecode>,
    pub constants: Vec<Value>,
    pub parameters: Vec<String>,
    pub name: Option<String>,
}
