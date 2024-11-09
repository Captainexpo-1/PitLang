use crate::common::{Function, Value};
use crate::virtualmachine::bytecode::Bytecode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::format;
use std::rc::Rc;

type EnvironmentRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
struct Environment {
    values: HashMap<String, Value>,
    parent: Option<EnvironmentRef>,
}

impl Environment {
    fn new(parent: Option<EnvironmentRef>) -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            parent,
        }))
    }

    fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => {
                if let Some(parent_env) = &self.parent {
                    parent_env.borrow().get(name)
                } else {
                    None
                }
            }
        }
    }

    fn set(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,               // Stack for values during execution
    globals: HashMap<String, Value>, // Global variables
    call_stack: Vec<CallFrame>,      // Stack to keep track of function calls
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    function: Rc<Function>,      // The function being executed
    instruction_pointer: usize,  // The instruction pointer for this frame
    environment: EnvironmentRef, // Environment for this function
}
impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}

impl VM {
    /// Creates a new VM with an empty stack and no global variables.
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    /// Executes a function by setting up a new call frame and running its instructions.
    pub fn run(&mut self, function: Rc<Function>) -> Result<Value, String> {
        self.call_stack.push(CallFrame {
            function: function.clone(),
            instruction_pointer: 0,
            environment: Environment::new(None),
        });

        while let Some(frame) = self.call_stack.last_mut() {
            // Fetch the next instruction and execute it
            if frame.instruction_pointer >= frame.function.instructions.len() {
                self.call_stack.pop();
                break;
            }
            let instruction = frame.function.instructions[frame.instruction_pointer].clone();
            frame.instruction_pointer += 1;

            self.execute_instruction(instruction)?;
        }

        Ok(self.stack.pop().unwrap_or(Value::Unit))
    }
    #[inline]
    fn pop_stack(&mut self) -> Value {
        self.stack
            .pop()
            .unwrap_or_else(|| panic!("No value on the stack"))
    }

    fn execute_instruction(&mut self, instruction: Bytecode) -> Result<(), String> {
        match instruction {
            // Load a constant from the function's constant pool by index
            Bytecode::LoadConst(index) => {
                let frame = self.call_stack.last().ok_or("No call frame found")?;
                let constant = frame
                    .function
                    .constants
                    .get(index)
                    .cloned()
                    .ok_or("Constant not found")?;
                self.stack.push(constant);
            }
            // Load a variable from local or global scope
            Bytecode::LoadVar(name) => {
                if let Some(frame) = self.call_stack.last() {
                    if let Some(value) = frame.environment.borrow().get(&name) {
                        self.stack.push(value);
                    } else if let Some(value) = self.globals.get(&name).cloned() {
                        self.stack.push(value);
                    } else {
                        return Err(format!("Variable {} not found", name));
                    }
                } else {
                    return Err("No call frame found".to_string());
                }
            }
            // Store the top stack value into a variable
            Bytecode::StoreVar(name) => {
                let value = self.pop_stack();
                if let Some(frame) = self.call_stack.last() {
                    frame.environment.borrow_mut().set(name, value);
                } else {
                    self.globals.insert(name, value);
                }
            }
            // Arithmetic operations
            Bytecode::Add => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.add_values(a, b)?);
            }
            Bytecode::Sub => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.sub_values(a, b)?);
            }
            Bytecode::Mul => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.mul_values(a, b)?);
            }
            Bytecode::Div => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.div_values(a, b)?);
            }
            // Comparison operations
            Bytecode::Eq => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(Value::Boolean(a == b));
            }
            Bytecode::NotEq => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(Value::Boolean(a != b));
            }
            Bytecode::Lt => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.lt_values(a, b)?);
            }
            Bytecode::Gt => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.gt_values(a, b)?);
            }
            Bytecode::LtEqual => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.lte_values(a, b)?);
            }
            Bytecode::GtEqual => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.gte_values(a, b)?);
            }
            // Control flow
            Bytecode::Jump(position) => {
                let frame = self.call_stack.last_mut().ok_or("No call frame found")?;
                frame.instruction_pointer = position;
            }
            Bytecode::JumpIfFalse(position) => {
                let condition = self.pop_stack();
                if !condition.is_truthy() {
                    let frame = self.call_stack.last_mut().ok_or("No call frame found")?;
                    frame.instruction_pointer = position;
                }
            }
            // Function call
            Bytecode::Call(arg_count) => {
                // Collect arguments from the stack
                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    args.push(self.pop_stack());
                }
                // Pop the function from the stack
                let function_value = self.pop_stack();

                // Reverse arguments to maintain correct order
                args.reverse();

                // Check if the value is a function
                if let Value::Function(func) = function_value {
                    // Get the current frame's environment as the parent
                    let parent_environment = self
                        .call_stack
                        .last()
                        .map(|frame| frame.environment.clone());

                    // Create a new environment with the parent
                    let environment = Environment::new(parent_environment);

                    // Set up parameters in the new environment
                    {
                        let mut env_mut = environment.borrow_mut();
                        for (i, param) in func.parameters.iter().enumerate() {
                            let arg = args.get(i).cloned().unwrap_or(Value::Null);
                            env_mut.set(param.clone(), arg);
                        }
                    }

                    // Create the new call frame
                    let frame = CallFrame {
                        function: func.clone(),
                        instruction_pointer: 0,
                        environment,
                    };
                    self.call_stack.push(frame);
                } else {
                    return Err(format!(
                        "Attempted to call a non-function: {:?}",
                        function_value
                    ));
                }
            }
            // Function return
            Bytecode::Return => {
                let return_value = self.pop_stack();
                self.call_stack.pop();
                self.stack.push(return_value);
            }
            Bytecode::GetProp(property) => {
                let object = self.pop_stack();
                match object {
                    Value::Object(obj) => {
                        let obj = obj.borrow();
                        if let Some(value) = obj.get(&property) {
                            self.stack.push(value.clone());
                        } else {
                            return Err(format!("Property {} not found", property));
                        }
                    }
                    _ => return Err("Value is not an object".to_string()),
                }
            }
            Bytecode::SetProp(property) => {
                let value = self.pop_stack();
                let object = self.pop_stack();
                match object {
                    Value::Object(obj) => {
                        obj.borrow_mut().insert(property, value);
                    }
                    _ => return Err("Value is not an object".to_string()),
                }
            }
            // Unknown instruction handling
            _ => return Err(format!("Unknown instruction: {:?}", instruction)),
        }
        Ok(())
    }

    /// Helper function for addition of two values.
    fn add_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
            _ => Err("Type error in addition".to_string()),
        }
    }
    fn sub_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
            _ => Err("Type error in subtraction".to_string()),
        }
    }
    fn mul_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
            _ => Err("Type error in multiplication".to_string()),
        }
    }
    fn div_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
            _ => Err("Type error in division".to_string()),
        }
    }
    fn lt_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x < y)),
            _ => Err("Type error in less than comparison".to_string()),
        }
    }
    fn gt_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x > y)),
            _ => Err("Type error in greater than comparison".to_string()),
        }
    }
    fn lte_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x <= y)),
            _ => Err("Type error in less than or equal comparison".to_string()),
        }
    }
    fn gte_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
            _ => Err("Type error in greater than or equal comparison".to_string()),
        }
    }
}
