use crate::treewalk::evaluator::runtime_error;
use crate::treewalk::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type StdMethod = fn(&Value, Vec<Value>) -> Value;

pub fn std_methods() -> HashMap<String, StdMethod> {
    // For the included 'std' object, E.G. std.time()
    let mut methods: HashMap<String, StdMethod> = HashMap::new();
    methods.insert("time".to_string(), |_this: &Value, _args: Vec<Value>| {
        Value::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        )
    });
    methods.insert("random".to_string(), |_this: &Value, _args: Vec<Value>| {
        Value::Number(rand::random::<f64>())
    });
    methods.insert("print".to_string(), |_this: &Value, args: Vec<Value>| {
        for (i, arg) in args.iter().enumerate() {
            arg.print();
            if i < args.len() - 1 {
                print!(" ");
            }
        }
        Value::Null
    });
    methods.insert("println".to_string(), |_this: &Value, args: Vec<Value>| {
        for (i, arg) in args.iter().enumerate() {
            arg.print();
            if i < args.len() - 1 {
                print!(" ");
            }
            println!();
        }
        Value::Null
    });
    methods.insert("argv".to_string(), |_this: &Value, _args: Vec<Value>| {
        let args: Vec<Value> = std::env::args().map(Value::String).collect();
        Value::Array(Rc::new(RefCell::new(args)))
    });
    methods.insert("exit".to_string(), |_this: &Value, args: Vec<Value>| {
        if let Value::Number(code) = args.first().unwrap_or(&Value::Null) {
            std::process::exit(*code as i32);
        } else {
            runtime_error("exit() argument must be a number")
        }
    });
    methods
}

pub fn string_methods() -> HashMap<String, StdMethod> {
    let mut methods: HashMap<String, StdMethod> = HashMap::new();
    methods.insert("length".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            Value::Number(s.len() as f64)
        } else {
            runtime_error("`length` method called on non-string value")
        }
    });
    methods.insert("ord".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            if s.len() == 1 {
                Value::Number(s.chars().next().unwrap() as u32 as f64)
            } else {
                runtime_error("ord() called on string with length != 1")
            }
        } else {
            runtime_error("`ord` method called on non-string value")
        }
    });
    methods.insert("get".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Value::Number(i) = args[0] {
                let i = i as i64;
                // negative indices count from the end
                let i = if i < 0 { s.len() as i64 + i } else { i };
                if i >= 0 && i < s.len() as i64 {
                    Value::String(s.chars().nth(i as usize).unwrap().to_string())
                } else {
                    runtime_error("Index out of bounds")
                }
            } else {
                runtime_error("Index must be a number")
            }
        } else {
            runtime_error("`get` method called on non-string value")
        }
    });
    methods.insert("to_int".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Ok(n) = s.parse::<f64>() {
                Value::Number(n)
            } else {
                runtime_error("Could not parse string to number")
            }
        } else {
            runtime_error("`to_int` method called on non-string value")
        }
    });
    methods.insert("to_float".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Ok(n) = s.parse::<f64>() {
                Value::Number(n)
            } else {
                runtime_error("Could not parse string to number")
            }
        } else {
            runtime_error("`to_float` method called on non-string value")
        }
    });
    methods.insert("replace".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            let mut s = s.clone();
            for i in 0.._args.len() / 2 {
                if let Value::String(a) = &_args[i * 2] {
                    if let Value::String(b) = &_args[i * 2 + 1] {
                        s = s.replace(a, b);
                    } else {
                        return runtime_error("replace arguments must be strings");
                    }
                } else {
                    return runtime_error("replace arguments must be strings");
                }
            }
            Value::String(s)
        } else {
            runtime_error("`replace` method called on non-string value")
        }
    });
    methods.insert("split".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Value::String(sep) = args.first().unwrap_or(&Value::String(" ".to_string())) {
                let parts: Vec<Value> =
                    s.split(sep).map(|s| Value::String(s.to_string())).collect();
                Value::Array(Rc::new(RefCell::new(parts)))
            } else {
                runtime_error("split argument must be a string")
            }
        } else {
            runtime_error("`split` method called on non-string value")
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
    methods.insert("round".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.round())
        } else {
            runtime_error("`round` method called on non-number value")
        }
    });
    methods.insert("floor".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.floor())
        } else {
            runtime_error("`floor` method called on non-number value")
        }
    });
    methods.insert("ceil".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.ceil())
        } else {
            runtime_error("`ceil` method called on non-number value")
        }
    });
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
    methods.insert("remove".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Value::Number(i) = args[0] {
                let i = i as usize;
                if i < a.borrow().len() {
                    let removed = a.borrow_mut().remove(i);
                    removed
                } else {
                    runtime_error("Index out of bounds")
                }
            } else {
                runtime_error("Index must be a number")
            }
        } else {
            runtime_error("`remove` method called on non-array value")
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
                let i = i as i64;
                // negative indices count from the end
                let i = if i < 0 {
                    a.borrow().len() as i64 + i
                } else {
                    i
                };
                if i >= 0 && i < a.borrow().len() as i64 {
                    a.borrow()[i as usize].clone()
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
    methods.insert("pop".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Some(v) = a.borrow_mut().pop() {
                v
            } else {
                Value::Null
            }
        } else {
            runtime_error("`pop` method called on non-array value")
        }
    });
    methods.insert("find".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Some(i) = a.borrow().iter().position(|v| v == &_args[0]) {
                Value::Number(i as f64)
            } else {
                Value::Number(-1.)
            }
        } else {
            runtime_error("`find` method called on non-array value")
        }
    });
    methods
}
