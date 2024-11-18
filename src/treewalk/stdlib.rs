use crate::treewalk::evaluator::runtime_error;
use crate::treewalk::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
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
        for arg in args.iter() {
            arg.print();
        }
        // Flush stdout
        std::io::stdout().flush().unwrap();
        Value::Null
    });
    methods.insert("println".to_string(), |_this: &Value, args: Vec<Value>| {
        for arg in args.iter() {
            arg.print();
        }
        println!();
        Value::Null
    });
    methods.insert("argv".to_string(), |_this: &Value, _args: Vec<Value>| {
        let args: Vec<Value> = std::env::args().map(Value::String).collect();
        Value::Array(Rc::new(RefCell::new(args)))
    });
    methods.insert(
        "get_line".to_string(),
        |_this: &Value, _args: Vec<Value>| {
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                eprintln!("Error reading input: {}", e);
                Value::Null
            } else {
                Value::String(input)
            }
        },
    );

    methods.insert(
        "write_file".to_string(),
        |_this: &Value, args: Vec<Value>| {
            if let Value::String(file) = &args[0] {
                if let Value::String(contents) = &args[1] {
                    if let Ok(mut file) = std::fs::File::create(file) {
                        if let Err(e) = file.write_all(contents.as_bytes()) {
                            eprintln!("Error writing to file: {}", e);
                        }
                    } else {
                        eprintln!("Error creating file");
                    }
                    Value::Null
                } else {
                    runtime_error(
                        format!("write_file contents must be a string: got {:?}", args[1]).as_str(),
                    )
                }
            } else {
                runtime_error(
                    format!("write_file file path must be a string: got {:?}", args[0]).as_str(),
                )
            }
        },
    );

    methods.insert(
        "read_file".to_string(),
        |_this: &Value, args: Vec<Value>| {
            if let Value::String(file) = &args[0] {
                match std::fs::read_to_string(file) {
                    Ok(contents) => Value::String(contents),
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        Value::Null
                    }
                }
            } else {
                runtime_error(
                    format!("read_file file path must be a string: got {:?}", args[0]).as_str(),
                )
            }
        },
    );

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
            runtime_error(
                format!(
                    "`length` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
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
            runtime_error(
                format!(
                    "`ord` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("get".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Value::Number(i) = args[0] {
                let i = i as i64;
                if i >= 0 && i < s.len() as i64 {
                    Value::String(s.chars().nth(i as usize).unwrap().to_string())
                } else {
                    runtime_error(
                        format!(
                            "Index out of bounds in `get` method: index {}, length {}",
                            i,
                            s.len(),
                        )
                        .as_str(),
                    )
                }
            } else {
                runtime_error(
                    format!("Index must be a number in `get` method: got {:?}", args[0]).as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`get` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("to_int".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Ok(n) = s.parse::<f64>() {
                Value::Number(n)
            } else {
                runtime_error(
                    format!(
                        "Could not parse string to number in `to_int` method: got {:?}",
                        s,
                    )
                    .as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`to_int` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("to_float".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Ok(n) = s.parse::<f64>() {
                Value::Number(n)
            } else {
                runtime_error(
                    format!(
                        "Could not parse string to number in `to_float` method: got {:?}",
                        s,
                    )
                    .as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`to_float` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
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
                        return runtime_error(
                            format!(
                                "replace arguments must be strings: got {:?}",
                                _args[i * 2 + 1],
                            )
                            .as_str(),
                        );
                    }
                } else {
                    return runtime_error(
                        format!("replace arguments must be strings: got {:?}", _args[i * 2],)
                            .as_str(),
                    );
                }
            }
            Value::String(s)
        } else {
            runtime_error(
                format!(
                    "`replace` method called on non-string value: expected String, got {:?}",
                    this
                )
                .as_str(),
            )
        }
    });
    methods.insert("split".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Value::String(sep) = args.first().unwrap_or(&Value::String(" ".to_string())) {
                let parts: Vec<Value> =
                    s.split(sep).map(|s| Value::String(s.to_string())).collect();
                Value::Array(Rc::new(RefCell::new(parts)))
            } else {
                runtime_error(
                    format!("split argument must be a string: got {:?}", args.first()).as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`split` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("find".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::String(s) = this {
            if let Some(i) = s.find(if let Value::String(s) = &args[0] {
                s
            } else {
                return runtime_error(
                    format!(
                        "`find` method called with non-string argument: expected String, got {:?}",
                        args[0]
                    )
                    .as_str(),
                );
            }) {
                Value::Number(i as f64)
            } else {
                Value::Number(-1.)
            }
        } else {
            runtime_error(
                format!(
                    "`find` method called on non-string value: expected String, got {:?}",
                    this,
                )
                .as_str(),
            )
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
                runtime_error(
                    format!(
                        "`to_string` method called on non-number value: expected Number, got {:?}",
                        this,
                    )
                    .as_str(),
                )
            }
        },
    );
    methods.insert("round".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.round())
        } else {
            runtime_error(
                format!(
                    "`round` method called on non-number value: expected Number, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("floor".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.floor())
        } else {
            runtime_error(
                format!(
                    "`floor` method called on non-number value: expected Number, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("ceil".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Number(n) = this {
            Value::Number(n.ceil())
        } else {
            runtime_error(
                format!(
                    "`ceil` method called on non-number value: expected Number, got {:?}",
                    this,
                )
                .as_str(),
            )
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
            runtime_error(
                format!(
                    "`length` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
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
                    runtime_error(
                        format!(
                            "Index out of bounds in `remove` method: index {}, length {}",
                            i,
                            a.borrow().len(),
                        )
                        .as_str(),
                    )
                }
            } else {
                runtime_error(
                    format!(
                        "Index must be a number in `remove` method: got {:?}",
                        args[0],
                    )
                    .as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`remove` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("push".to_string(), |this: &Value, args: Vec<Value>| {
        if let Value::Array(a) = this {
            a.borrow_mut().push(args[0].clone());
            Value::Null
        } else {
            runtime_error(
                format!(
                    "`push` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
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
                    runtime_error(
                        format!(
                            "Index out of bounds in `set` method: index {}, length {}",
                            i,
                            a.borrow().len(),
                        )
                        .as_str(),
                    )
                }
            } else {
                runtime_error(
                    format!("Index must be a number in `set` method: got {:?}", args[0]).as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`set` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
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
                    runtime_error(
                        format!(
                            "Index out of bounds in `get` method: index {}, length {}",
                            i,
                            a.borrow().len(),
                        )
                        .as_str(),
                    )
                }
            } else {
                runtime_error(
                    format!("Index must be a number in `get` method: got {:?}", args[0]).as_str(),
                )
            }
        } else {
            runtime_error(
                format!(
                    "`get` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods.insert("pop".to_string(), |this: &Value, _args: Vec<Value>| {
        if let Value::Array(a) = this {
            if let Some(v) = a.borrow_mut().pop() {
                v
            } else {
                runtime_error("pop() called on empty array")
            }
        } else {
            runtime_error(
                format!(
                    "`pop` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
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
            runtime_error(
                format!(
                    "`find` method called on non-array value: expected Array, got {:?}",
                    this,
                )
                .as_str(),
            )
        }
    });
    methods
}
