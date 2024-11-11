use crate::virtualmachine::bytecode::{Bytecode, Instruction};
use crate::virtualmachine::value::{CallFrame, Value};

const STACK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    running: bool,
    call_stack: Vec<CallFrame>,
    ip: usize,
    sp: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::with_capacity(STACK_SIZE),
            instructions: Vec::new(),
            constants: Vec::new(),
            running: false,
            call_stack: Vec::new(),
            ip: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self, bytecode: Bytecode) {
        (self.instructions, self.constants) = (bytecode.instructions, bytecode.constants);
        self.running = true;
        self.call_stack.push(CallFrame {
            is_main: true,
            return_address: 0,
            locals: Vec::new(),
        });
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
                let constant = self.constants[index].clone();
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
            Instruction::Jmp(offset) => {
                self.ip = offset;
            }
            Instruction::Jit(offset) => {
                let condition = self.pop_stack();
                if condition.is_truthy() {
                    self.ip = offset;
                } else {
                    self.ip += 1;
                }
            }
            Instruction::Jif(offset) => {
                let condition = self.pop_stack();
                if !condition.is_truthy() {
                    self.ip = offset;
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
            Instruction::Not => {
                let value = self.pop_stack();
                let result = Value::Bool(!value.is_truthy());
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Return => {
                // Return the function result to the caller
                let result = self.pop_stack();

                // Pop the current frame and restore the caller frame
                let frame = self.call_stack.pop().unwrap();
                if frame.is_main {
                    self.running = false;
                    return;
                }
                self.ip = frame.return_address;

                // Push the result back on the stack for caller
                self.push_stack(result);
            }
            Instruction::Call(func_index) => {
                // Retrieve function metadata from constants
                let func_meta = match &self.constants[func_index] {
                    Value::Function(meta) => meta.clone(),
                    _ => {
                        println!("Call to non-function value");
                        self.running = false;
                        return;
                    }
                };

                // Pop arguments from stack and prepare new CallFrame
                let mut frame = CallFrame {
                    is_main: false,
                    return_address: self.ip + 1,
                    locals: vec![Value::Null; func_meta.arity], // Initialize locals with argument space
                };

                // Store arguments in frame.locals
                for i in (0..func_meta.arity).rev() {
                    frame.locals[i] = self.pop_stack();
                }

                // Push the new frame and jump to function address
                self.call_stack.push(frame);
                self.ip = func_meta.address;
            }
            Instruction::TypeOf => {
                let value = self.pop_stack();
                let result = Value::String(value.get_type());
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::IsNull => {
                let value = self.pop_stack();
                let result = Value::Bool(value == Value::Null);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::IsUndefined => {
                let value = self.pop_stack();
                let result = Value::Bool(value == Value::Undefined);
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Swap => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                self.push_stack(a);
                self.push_stack(b);
                self.ip += 1;
            }
            Instruction::Negate => {
                let value = self.pop_stack();
                let result = match value {
                    Value::Number(n) => Value::Number(-n),
                    _ => {
                        println!("Unsupported operand type for negation");
                        Value::Null
                    }
                };
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::Not => {
                let value = self.pop_stack();
                let result = match value {
                    Value::Bool(b) => Value::Bool(!b),
                    _ => {
                        println!("Unsupported operand type for logical NOT");
                        Value::Null
                    }
                };
                self.push_stack(result);
                self.ip += 1;
            }
            Instruction::StoreLocal(index) => {
                println!("StoreLocal: {} {}", index, self.ip);
                println!("Call stack: {:?}", self.call_stack);
                let value = self.pop_stack();
                if let Some(frame) = self.call_stack.last_mut() {
                    // Resize locals if index is out of bounds
                    if frame.locals.len() <= index {
                        frame.locals.resize(index + 1, Value::Null);
                    }
                    frame.locals[index] = value;
                } else {
                    println!("No call frame available for StoreLocal");
                    self.running = false;
                }
                self.ip += 1;
            }
            Instruction::LoadLocal(index) => {
                if let Some(frame) = self.call_stack.last() {
                    if let Some(value) = frame.locals.get(index) {
                        self.push_stack(value.clone());
                    } else {
                        println!("Local variable index out of bounds");
                        self.running = false;
                    }
                } else {
                    println!("No call frame available for LoadLocal");
                    self.running = false;
                }
                self.ip += 1;
            }

            Instruction::DEBUG_LABEL(_) => self.ip += 1,
            _ => {
                println!("Unsupported instruction {:?}", instruction);
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
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
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
