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
