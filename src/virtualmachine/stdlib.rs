use crate::common::{StdMethod, Value};
use crate::treewalk::evaluator::runtime_error;
use std::collections::HashMap;

pub fn std_lib() -> HashMap<&'static str, StdMethod> {
    let mut stdlib: HashMap<&'static str, StdMethod> = HashMap::new();
    stdlib.insert("print", _print);
    stdlib.insert("println", _println);
    stdlib.insert("system", _system);
    stdlib
}

pub fn _print(_receiver: &Value, args: Vec<Value>) -> Value {
    for arg in args {
        arg.print();
    }
    Value::Null
}

pub fn _println(_receiver: &Value, args: Vec<Value>) -> Value {
    _print(_receiver, args);
    println!();
    Value::Null
}

pub fn _system(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::String(s) = &args[0] {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(s)
            .output()
            .expect("Failed to execute command");
        let stdout = String::from_utf8_lossy(&output.stdout);
        Value::String(stdout.to_string())
    } else {
        runtime_error("Expected string")
    }
}

// String methods
pub fn string_methods() -> HashMap<&'static str, StdMethod> {
    let mut methods: HashMap<&'static str, StdMethod> = HashMap::new();
    methods.insert("parse_number", _string_parse_number);
    methods.insert("at", _string_getchar);
    methods
}

pub fn _string_parse_number(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::String(s) = &args[0] {
        if let Ok(n) = s.parse::<f64>() {
            Value::Number(n)
        } else {
            runtime_error("Failed to parse number")
        }
    } else {
        runtime_error("Expected string")
    }
}

pub fn _string_getchar(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::String(s) = &args[0] {
        if let Value::Number(n) = args[1] {
            let n = n as usize;
            if n < s.len() {
                Value::String(s.chars().nth(n).unwrap().to_string())
            } else {
                runtime_error("Index out of bounds")
            }
        } else {
            runtime_error("Expected number")
        }
    } else {
        runtime_error("Expected string")
    }
}

// Number methods
pub fn number_methods() -> HashMap<&'static str, StdMethod> {
    let mut methods: HashMap<&'static str, StdMethod> = HashMap::new();
    methods.insert("to_string", _number_to_string);
    methods
}

pub fn _number_to_string(_receiver: &Value, _args: Vec<Value>) -> Value {
    if let Value::Number(n) = _receiver {
        Value::String(n.to_string())
    } else {
        runtime_error("Expected number")
    }
}

// Array methods
pub fn array_methods() -> HashMap<&'static str, Value> {
    let mut methods: HashMap<&'static str, Value> = HashMap::new();
    methods.insert("length", Value::StdFunction(_array_length));
    methods.insert("push", Value::StdFunction(_array_push));
    methods.insert("get", Value::StdFunction(_array_get));
    methods.insert("set", Value::StdFunction(_array_set));
    methods
}

pub fn _array_length(_receiver: &Value, _args: Vec<Value>) -> Value {
    if let Value::Array(arr) = _receiver {
        Value::Number(arr.borrow().len() as f64)
    } else {
        runtime_error("Expected array")
    }
}

pub fn _array_push(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::Array(arr) = _receiver {
        arr.borrow_mut().push(args[0].clone());
        Value::Null
    } else {
        runtime_error("Expected array")
    }
}

pub fn _array_get(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::Array(arr) = _receiver {
        if let Value::Number(n) = args[0] {
            let n = n as usize;
            if n < arr.borrow().len() {
                arr.borrow()[n].clone()
            } else {
                runtime_error("Index out of bounds")
            }
        } else {
            runtime_error("Expected number")
        }
    } else {
        runtime_error("Expected array")
    }
}

pub fn _array_set(_receiver: &Value, args: Vec<Value>) -> Value {
    if let Value::Array(arr) = _receiver {
        if let Value::Number(n) = args[0] {
            let n = n as usize;
            if n < arr.borrow().len() {
                arr.borrow_mut()[n] = args[1].clone();
                Value::Null
            } else {
                runtime_error("Index out of bounds")
            }
        } else {
            runtime_error("Expected number")
        }
    } else {
        runtime_error("Expected array")
    }
}
