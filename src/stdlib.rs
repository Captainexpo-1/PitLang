use crate::evaluator::{runtime_error, Value};
use std::collections::HashMap;

pub type StdMethod = fn(&Value, Vec<Value>) -> Value;

pub fn string_methods() -> HashMap<String, StdMethod> {
    let mut methods: HashMap<String, StdMethod> = HashMap::new();
    methods.insert("length".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            Value::Number(s.len() as f64)
        } else {
            runtime_error("`length` method called on non-string value")
        }
    });
    methods
}

pub fn number_methods() -> HashMap<String, StdMethod> {
    let mut methods: HashMap<String, StdMethod> = HashMap::new();
    methods.insert(
        "to_string".to_string(),
        |this: &Value, _args: Vec<Value>| {
            if let Value::Number(n) = this {
                Value::String(n.to_string())
            } else {
                runtime_error("`to_string` method called on non-number value")
            }
        },
    );
    methods
}

pub fn array_methods() -> HashMap<String, StdMethod> {
    let mut methods: HashMap<String, StdMethod> = HashMap::new();

    methods.insert("length".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Array(a) = this {
            Value::Number(a.borrow().len() as f64)
        } else {
            runtime_error("`length` method called on non-array value")
        }
    });
    methods.insert("push".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::Array(a) = this {
            a.borrow_mut().push(args[0].clone());
            Value::Null
        } else {
            runtime_error("`push` method called on non-array value")
        }
    });
    methods.insert("set".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Value::Number(i) = args[0] {
                let i = i as usize;
                if i < a.borrow().len() {
                    a.borrow_mut()[i] = args[1].clone();
                    Value::Null
                } else {
                    runtime_error("Index out of bounds")
                }
            } else {
                runtime_error("Index must be a number")
            }
        } else {
            runtime_error("`set` method called on non-array value")
        }
    });
    methods.insert("get".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Value::Number(i) = args[0] {
                let i = i as usize;
                if i < a.borrow().len() {
                    a.borrow_mut()[i].clone()
                } else {
                    runtime_error("Index out of bounds")
                }
            } else {
                runtime_error("Index must be a number")
            }
        } else {
            runtime_error("`get` method called on non-array value")
        }
    });
    methods
}
