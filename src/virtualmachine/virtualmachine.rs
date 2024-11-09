use crate::common::{Function, Instruction, Value};
use std::collections::HashMap;
pub struct VM {
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
    ip: usize, // Instruction pointer
}

impl VM {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        VM {
            instructions,
            constants,
            globals: HashMap::new(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn run(&mut self) -> Result<Value, String> {
        while self.ip < self.instructions.len() {
            let instruction = &self.instructions[self.ip];
            self.ip += 1;
            match instruction {
                Instruction::LoadConst(index) => {
                    let value = self
                        .constants
                        .get(*index)
                        .ok_or(format!("Constant at index {} not found", index))?
                        .clone();
                    self.stack.push(value);
                }
                Instruction::LoadNull => {
                    self.stack.push(Value::Null);
                }
                Instruction::Pop => {
                    self.stack.pop().ok_or("Stack underflow")?;
                }
                Instruction::LoadGlobal(name) => {
                    let value = self
                        .globals
                        .get(name)
                        .ok_or(format!("Undefined variable '{}'", name))?
                        .clone();
                    self.stack.push(value);
                }
                Instruction::StoreGlobal(name) => {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    self.globals.insert(name.clone(), value);
                }
                // Arithmetic operations
                Instruction::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b))
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::String(a + &b))
                        }
                        _ => return Err("Type error in Add operation".to_string()),
                    }
                }
                Instruction::Subtract => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a - b))
                        }
                        _ => return Err("Type error in Subtract operation".to_string()),
                    }
                }
                Instruction::Multiply => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a * b))
                        }
                        _ => return Err("Type error in Multiply operation".to_string()),
                    }
                }
                Instruction::Divide => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(_), Value::Number(0.0)) => {
                            return Err("Division by zero".to_string())
                        }
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a / b))
                        }
                        _ => return Err("Type error in Divide operation".to_string()),
                    }
                }
                Instruction::Modulo => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a % b))
                        }
                        _ => return Err("Type error in Modulo operation".to_string()),
                    }
                }
                // Comparison operations
                Instruction::Equal => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(Value::Boolean(a == b));
                }
                Instruction::NotEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(Value::Boolean(a != b));
                }
                Instruction::Greater => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a > b))
                        }
                        _ => return Err("Type error in Greater operation".to_string()),
                    }
                }
                Instruction::GreaterEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a >= b))
                        }
                        _ => return Err("Type error in GreaterEqual operation".to_string()),
                    }
                }
                Instruction::Less => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a < b))
                        }
                        _ => return Err("Type error in Less operation".to_string()),
                    }
                }
                Instruction::LessEqual => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a <= b))
                        }
                        _ => return Err("Type error in LessEqual operation".to_string()),
                    }
                }
                // Logical operations
                Instruction::And => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack
                        .push(Value::Boolean(a.is_truthy() && b.is_truthy()));
                }
                Instruction::Or => {
                    let b = self.stack.pop().ok_or("Stack underflow")?;
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack
                        .push(Value::Boolean(a.is_truthy() || b.is_truthy()));
                }
                Instruction::Not => {
                    let a = self.stack.pop().ok_or("Stack underflow")?;
                    self.stack.push(Value::Boolean(!a.is_truthy()));
                }
                // Control flow
                Instruction::Jump(target) => {
                    self.ip = *target;
                }
                Instruction::JumpIfFalse(target) => {
                    let condition = self.stack.pop().ok_or("Stack underflow")?;
                    if !condition.is_truthy() {
                        self.ip = *target;
                    }
                }
                Instruction::JumpIfTrue(target) => {
                    let condition = self.stack.pop().ok_or("Stack underflow")?;
                    if condition.is_truthy() {
                        self.ip = *target;
                    }
                }
                // Functions
                Instruction::Call(arg_count) => {
                    // For simplicity, handling built-in functions like 'print'
                    let func = self.stack.pop().ok_or("Stack underflow")?;
                    let args = self.stack.split_off(self.stack.len() - arg_count);
                    match func {
                        Value::String(ref name) if name == "print" => {
                            for arg in args {
                                arg.print();
                            }
                            println!();
                            self.stack.push(Value::Unit);
                        }
                        Value::Function(function) => {
                            // Handle user-defined function calls
                            // Create a new VM instance or manage call frames
                            return Err("User-defined functions not implemented yet".to_string());
                        }
                        _ => return Err("Unknown function or not callable".to_string()),
                    }
                }
                Instruction::Return => {
                    return Ok(self.stack.pop().unwrap_or(Value::Null));
                }
                // Miscellaneous
                Instruction::Print => {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    value.print();
                    println!();
                    self.stack.push(Value::Unit);
                }
            }
        }
        Ok(Value::Unit)
    }
}
