use crate::virtualmachine::bytecode::{Bytecode, Instruction};
use crate::virtualmachine::value::Value;
use std::collections::HashMap;
use std::result;

use super::bytecode;

const STACK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    running: bool,
    ip: usize,
    sp: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::with_capacity(STACK_SIZE),
            instructions: Vec::new(),
            constants: Vec::new(),
            running: false,
            ip: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self, bytecode: Bytecode) {
        (self.instructions, self.constants) = (bytecode.instructions, bytecode.constants);
        self.running = true;

        while self.running {
            self.execute_instruction();
        }
    }

    pub fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
        self.sp += 1;
    }

    pub fn pop_stack(&mut self) -> Value {
        if self.sp == 0 {
            println!("Stack underflow");
            self.running = false;
            return Value::Null;
        }
        self.sp -= 1;
        self.stack.pop().unwrap()
    }

    pub fn cur_instruction(&self) -> &Instruction {
        &self.instructions[self.ip]
    }

    pub fn execute_instruction(&mut self) {
        let instruction = self.cur_instruction();

        match *instruction {
            Instruction::Halt => self.running = false,
            Instruction::PushConst(index) => {
                let constant = self.constants[index as usize].clone();
                self.push_stack(constant);
                self.ip += 1;
            }
            Instruction::Add => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = self.add_values(a, b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Sub => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = self.sub_values(a, b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Mul => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = self.mul_values(a, b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Div => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = self.div_values(a, b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Mod => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = self.mod_values(a, b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Dup => {
                let value = self.stack[self.sp - 1].clone();
                self.push_stack(value);
                self.ip += 1;
            }
            Instruction::Pop => {
                self.pop_stack();
                self.ip += 1;
            }
            Instruction::Swap => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                self.push_stack(a);
                self.push_stack(b);
                self.ip += 1;
            }
            Instruction::Jmp(offset) => {
                self.ip = offset as usize;
            }
            Instruction::Jit(offset) => {
                let condition = self.pop_stack();
                if condition.is_truthy() {
                    self.ip = offset as usize;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Jif(offset) => {
                let condition = self.pop_stack();
                if !condition.is_truthy() {
                    self.ip = offset as usize;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Eq => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a == b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Ne => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a != b);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Gt => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a.greater_than(&b));
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Ge => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a.greater_than_or_equal(&b));
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Lt => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a.less_than(&b));
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Le => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                let result = Value::Bool(a.less_than_or_equal(&b));
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Return => {
                let return_address = self.pop_stack();
                self.ip = match return_address {
                    Value::Number(n) => n as usize,
                    _ => {
                        println!("Invalid return address");
                        self.running = false;
                        0
                    }
                };
            }
            _ => {
                println!("Unsupported instruction");
                self.running = false;
            }
        }
    }

    pub fn dump_stack(&self) {
        println!("Stack:");
        for value in &self.stack {
            value.print();
            println!();
        }
    }

    pub fn add_values(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => {
                println!("Unsupported operand types for addition");
                Value::Null
            }
        }
    }
    pub fn sub_values(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => {
                println!("Unsupported operand types for subtraction");
                Value::Null
            }
        }
    }
    pub fn mul_values(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => {
                println!("Unsupported operand types for multiplication");
                Value::Null
            }
        }
    }
    pub fn div_values(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => {
                println!("Unsupported operand types for division");
                Value::Null
            }
        }
    }
    pub fn mod_values(&self, a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => {
                println!("Unsupported operand types for modulo");
                Value::Null
            }
        }
    }
}
