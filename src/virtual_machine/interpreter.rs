use crate::virtual_machine::bytecode::{Bytecode, OpCode};
use crate::virtual_machine::value::Value;
#[derive(Default)]
pub struct Interpreter {
    stack: Vec<Value>,
    bytecode: Bytecode,
    ip: usize,
    halted: bool,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            stack: Vec::new(),
            bytecode,
            ip: 0,
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.ip = 0;
        self.halted = false;
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        if let Some(value) = self.stack.pop() {
            value
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn evaluate(&mut self) -> Result<Value, String> {
        self.halted = false;
        let mut disable_increment = false;
        while self.ip < self.bytecode.code.len() && !self.halted {
            println!("{:?}", self.stack);
            let op = self.bytecode.code[self.ip].clone();
            disable_increment = false;
            match op {
                OpCode::ADD => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(a + b);
                }
                OpCode::SUB => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(a - b);
                }
                OpCode::MUL => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(a * b);
                }
                OpCode::DIV => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(a / b);
                }
                OpCode::EQ => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a == b));
                }
                OpCode::NEQ => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a != b));
                }
                OpCode::LT => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a < b));
                }
                OpCode::LTE => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a <= b));
                }
                OpCode::GT => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a > b));
                }
                OpCode::GTE => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::new_boolean(a >= b));
                }
                OpCode::POP => {
                    self.pop();
                }
                OpCode::HALT => {
                    self.halted = true;
                }
                OpCode::CONST(idx) => {
                    let value = self.bytecode.constants[idx as usize];
                    self.stack.push(value);
                }
                OpCode::JUMP_IF_FALSE(addr) => {
                    let condition = self.pop();
                    if !condition.is_truthy() {
                        self.ip = addr;
                        disable_increment = true;
                    }
                }
                OpCode::JUMP(addr) => {
                    self.ip = addr;
                    disable_increment = true;
                }
                _ => return Err(format!("Unknown opcode: {:?}", op)),
            }
            if !disable_increment {
                self.ip += 1
            };
        }
        Ok(self.pop())
    }
}
