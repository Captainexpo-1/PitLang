use crate::common::{Function, Value};
use crate::virtualmachine::bytecode::Bytecode;
use crate::virtualmachine::stdlib::array_methods;
use crate::virtualmachine::stdlib::std_lib;
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
        // Iterate through the environment chain to find the value without recursion
        let mut env = Some(Rc::new(RefCell::new(self.clone())));
        while let Some(current) = env {
            if let Some(value) = current.borrow().values.get(name) {
                return Some(value.clone());
            }
            env = current.borrow().parent.clone();
        }
        None
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
    // Creates a new VM with an empty stack and no global variables.
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    // Executes a function by setting up a new call frame and running its instructions.
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

    pub fn run_stdlib_function(&mut self, function: String, args: Vec<Value>) -> Option<Value> {
        if !std_lib().contains_key(function.as_str()) {
            return None;
        }
        Some(std_lib()[function.as_str()](&Value::Null, args))
    }

    #[inline]
    fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap()
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
                    } else if std_lib().contains_key(name.as_str()) {
                        self.stack.push(Value::Function(Rc::new(Function {
                            instructions: vec![],
                            constants: vec![],
                            parameters: vec![],
                            name: Some(name.clone()),
                        })));
                    } else {
                        return Err(format!("Variable {} not found during LoadVar", name));
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
            Bytecode::Mod => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(self.mod_values(a, b)?);
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
                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    args.push(self.pop_stack());
                }

                let function_value = self.pop_stack();
                let receiver = self.pop_stack();
                args.reverse();

                if let Value::Function(func) = function_value {
                    if func.name.is_some()
                        && std_lib().contains_key(func.name.as_ref().unwrap().as_str())
                    {
                        let return_value = self
                            .run_stdlib_function(func.name.as_ref().unwrap().clone(), args.clone());
                        if let Some(value) = return_value {
                            self.stack.push(value);
                            return Ok(());
                        }
                    }

                    let parent_environment = self
                        .call_stack
                        .last()
                        .map(|frame| frame.environment.clone());

                    let environment = Environment::new(parent_environment);

                    // Set up parameters in the new environment
                    {
                        let mut env_mut = environment.borrow_mut();
                        for (i, param) in func.parameters.iter().enumerate() {
                            let arg = args.get(i).cloned().unwrap_or(Value::Null);
                            env_mut.set(param.clone(), arg);
                        }
                    }

                    let frame = CallFrame {
                        function: func.clone(),
                        instruction_pointer: 0,
                        environment,
                    };
                    self.call_stack.push(frame);
                } else if let Value::StdFunction(func) = function_value {
                    let result = func(&receiver, args);
                    self.stack.push(result);
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
                    Value::Array(_) => {
                        let array_m = array_methods();
                        if let Some(method) = array_m.get(property.as_str()) {
                            self.stack.push(method.clone());
                        } else {
                            return Err(format!("Method {} not found", property));
                        }
                    }
                    _ => return Err(format!("Value is not an object {:?}", object).to_string()),
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
            Bytecode::BuildArray(count) => {
                let mut elements = Vec::with_capacity(count);
                for _ in 0..count {
                    elements.push(self.pop_stack());
                }
                elements.reverse(); // Reverse to maintain the correct order
                self.stack
                    .push(Value::Array(Rc::new(RefCell::new(elements))));
            }
            Bytecode::Duplicate => {
                let value = self.stack.last().cloned().ok_or("No value to duplicate")?;
                self.stack.push(value);
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
    fn mod_values(&self, a: Value, b: Value) -> Result<Value, String> {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x % y)),
            _ => Err("Type error in modulo".to_string()),
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
