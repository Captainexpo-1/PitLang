use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::ASTNode;

pub type StdMethod = fn(&Value, Vec<Value>) -> Value; // Takes a receiver and arguments, returns a value

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    variables: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent,
        }
    }
    pub fn insert(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.borrow().get(name))
    }
}

pub fn object_to_string(obj: &Value) {
    if let Value::Object(properties) = obj {
        print!("{{");
        for (i, (key, value)) in properties.borrow().iter().enumerate() {
            print!("{}: ", key);
            value.print();
            if i < properties.borrow().len() - 1 {
                print!(", ");
            }
        }
        print!("}}");
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Return(Box<Value>),
    Array(Rc<RefCell<Vec<Value>>>),
    Function {
        parameters: Vec<String>,
        body: Box<ASTNode>,
        env: Rc<RefCell<Scope>>,
    },
    RustFunction(StdMethod),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Method {
        receiver: Box<Value>,
        method_name: String,
    },
    Null,
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
            Value::Object(_) => object_to_string(self),
            Value::Function { .. } => print!("Function"),
            Value::Method {
                receiver,
                method_name,
            } => {
                print!("Method: {:?}.{}", receiver, method_name)
            }
            _ => print!("Unsupported value"),
        }
    }
}
