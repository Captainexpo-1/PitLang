use crate::ast::ASTNode;
use crate::tokenizer::TokenKind;
use crate::treewalk::stdlib::{array_methods, number_methods, string_methods};
use crate::treewalk::value::{Scope, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::stdlib::std_methods;

pub fn evaluate(program: &ASTNode) -> Value {
    let mut evaluator = TreeWalk::new(match program {
        ASTNode::Program(statements) => statements,
        _ => {
            runtime_error("Program node expected");
            return Value::Null;
        }
    });
    evaluator.evaluate_program()
}

pub fn runtime_error(msg: &str) -> Value {
    panic!("Runtime error: {}", msg);
}

type MethodMap = HashMap<String, fn(&Value, Vec<Value>) -> Value>;

struct TreeWalk<'a> {
    program: &'a Vec<ASTNode>,
    global_environment: Rc<RefCell<Scope>>,
    current_scope: Rc<RefCell<Scope>>,

    string_methods: MethodMap,
    number_methods: MethodMap,
    array_methods: MethodMap,
}

impl<'a> TreeWalk<'a> {
    pub fn new(program: &'a Vec<ASTNode>) -> Self {
        let global_env = Rc::new(RefCell::new(Scope::new(None)));
        TreeWalk {
            program,
            global_environment: global_env.clone(),
            current_scope: global_env,

            string_methods: HashMap::new(),
            number_methods: HashMap::new(),
            array_methods: HashMap::new(),
        }
    }

    fn evaluate_program(&mut self) -> Value {
        self.string_methods = string_methods();
        self.number_methods = number_methods();
        self.array_methods = array_methods();

        let mut std_map = HashMap::new();
        for method in std_methods() {
            std_map.insert(method.0.to_string(), Value::RustFunction(method.1));
        }
        self.global_environment.borrow_mut().insert(
            "std".to_string(),
            Value::Object(Rc::new(RefCell::new(std_map))),
        );

        let mut result = Value::Null;
        for stmt in self.program {
            result = self.evaluate_node(stmt);
            if let Value::Return(val) = result {
                return *val;
            }
        }
        result
    }

    fn evaluate_node(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::NumberLiteral(n) => Value::Number(*n),
            ASTNode::BooleanLiteral(b) => Value::Boolean(*b),
            ASTNode::NullLiteral => Value::Null,
            ASTNode::ObjectLiteral(properties) => {
                let mut obj = HashMap::new();
                for (key, val) in properties {
                    obj.insert(key.clone(), self.evaluate_node(val));
                }
                Value::Object(Rc::new(RefCell::new(obj)))
            }
            ASTNode::StringLiteral(s) => Value::String(s.clone()),
            ASTNode::ArrayLiteral(values) => {
                let mut arr = Vec::new();
                for val in values {
                    arr.push(self.evaluate_node(val));
                }
                Value::Array(Rc::new(RefCell::new(arr)))
            }
            ASTNode::Variable(name) => self
                .current_scope
                .borrow()
                .get(name)
                .unwrap_or_else(|| runtime_error(&format!("Undefined variable: {}", name))),
            ASTNode::VariableDeclaration { name, value } => {
                let val = self.evaluate_node(value);
                self.current_scope.borrow_mut().insert(name.clone(), val);
                Value::Null
            }
            ASTNode::Expression(expr) => self.evaluate_node(expr),
            ASTNode::BinaryOp { left, op, right } => self.evaluate_binary_op(op, left, right),
            ASTNode::UnaryOp { op, operand } => self.evaluate_unary_op(op, operand),
            ASTNode::MemberAccess { object, member } => {
                let obj_val = self.evaluate_node(object);

                if let Value::Object(properties) = obj_val {
                    let properties = properties.borrow();
                    match properties.get(member) {
                        Some(val) => val.clone(),
                        None => runtime_error(&format!(
                            "Property '{}' not found in object: {:?}",
                            member, properties
                        )),
                    }
                } else {
                    Value::Method {
                        receiver: Box::new(obj_val),
                        method_name: member.clone(),
                    }
                }
            }
            ASTNode::Block(statements) => {
                let previous_scope = self.current_scope.clone();
                self.current_scope =
                    Rc::new(RefCell::new(Scope::new(Some(previous_scope.clone()))));

                let mut result = Value::Null;
                for stmt in statements {
                    result = self.evaluate_node(stmt);
                    if let Value::Return(_) = result {
                        break;
                    }
                }

                self.current_scope = previous_scope;
                result
            }
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            } => {
                let cond = self.evaluate_node(condition);
                match cond {
                    Value::Boolean(true) => self.evaluate_node(consequence),
                    Value::Boolean(false) => {
                        if let Some(alt) = alternative {
                            self.evaluate_node(alt)
                        } else {
                            Value::Null
                        }
                    }
                    _ => runtime_error("Condition must be a boolean"),
                }
            }
            ASTNode::FunctionDeclaration {
                name,
                parameters,
                body,
            } => {
                let func = Value::Function {
                    parameters: parameters.clone(),
                    body: Box::new(*body.clone()),
                    env: self.current_scope.clone(),
                };

                if let Some(name) = name {
                    self.current_scope
                        .borrow_mut()
                        .insert(name.clone(), func.clone());
                    Value::Null
                } else {
                    func
                }
            }
            ASTNode::WhileStatement { condition, body } => {
                let mut result = Value::Null;
                while self.evaluate_node(condition).is_truthy() {
                    result = self.evaluate_node(body);
                    if let Value::Return(_) = result {
                        break;
                    }
                }
                result
            }
            ASTNode::FunctionCall { callee, arguments } => {
                let func = self.evaluate_node(callee);

                match func {
                    Value::Function {
                        parameters,
                        body,
                        env,
                    } => {
                        if parameters.len() != arguments.len() {
                            runtime_error("Argument count mismatch");
                        }

                        let new_scope = Rc::new(RefCell::new(Scope::new(Some(env.clone()))));
                        {
                            let mut scope_borrow = new_scope.borrow_mut();
                            for (param, arg) in parameters.iter().zip(arguments) {
                                let arg_val = self.evaluate_node(arg);
                                scope_borrow.insert(param.clone(), arg_val);
                            }
                        }

                        let previous_scope = self.current_scope.clone();
                        self.current_scope = new_scope;

                        let result = self.evaluate_node(&body);

                        self.current_scope = previous_scope;
                        if let Value::Return(val) = result {
                            *val
                        } else {
                            Value::Null
                        }
                    }
                    Value::Method {
                        receiver,
                        method_name,
                    } => self.call_method(
                        *receiver,
                        &method_name,
                        &arguments
                            .iter()
                            .map(|arg| Box::new(arg.clone()))
                            .collect::<Vec<_>>(),
                    ),
                    Value::RustFunction(func) => {
                        let args: Vec<Value> = arguments
                            .iter()
                            .map(|arg| self.evaluate_node(arg))
                            .collect();
                        func(&Value::Null, args)
                    }
                    _ => runtime_error("Called value is not a function"),
                }
            }

            ASTNode::ReturnStatement(expr) => {
                let val = self.evaluate_node(expr);
                Value::Return(Box::new(val))
            }
            _ => runtime_error(format!("Unsupported AST node: {:?}", node).as_str()),
        }
    }
    fn call_method(
        &mut self,
        receiver: Value,
        method_name: &str,
        arg_nodes: &[Box<ASTNode>],
    ) -> Value {
        let args: Vec<Value> = arg_nodes
            .iter()
            .map(|arg| self.evaluate_node(arg))
            .collect();
        let method = match &receiver {
            Value::String(_) => self.string_methods.get(method_name),
            Value::Number(_) => self.number_methods.get(method_name),
            Value::Array(_) => self.array_methods.get(method_name),
            _ => None,
        };

        if let Some(method) = method {
            method(&receiver, args)
        } else {
            runtime_error(&format!(
                "Method '{}' not found for {:?}",
                method_name, receiver
            ))
        }
    }
    fn bin_op_error(&self, op: &TokenKind, left: &Value, right: &Value) -> Value {
        runtime_error(&format!(
            "Unsupported binary operation: {:?} {:?} {:?}",
            left, op, right
        ))
    }
    fn evaluate_binary_op(&mut self, op: &TokenKind, left: &ASTNode, right: &ASTNode) -> Value {
        match op {
            TokenKind::And => {
                let left_val = self.evaluate_node(left);
                if !left_val.is_truthy() {
                    return Value::Boolean(false);
                }
                let right_val = self.evaluate_node(right);
                Value::Boolean(right_val.is_truthy())
            }
            TokenKind::Or => {
                let left_val = self.evaluate_node(left);
                if left_val.is_truthy() {
                    return Value::Boolean(true);
                }
                let right_val = self.evaluate_node(right);
                Value::Boolean(right_val.is_truthy())
            }
            _ => {
                let left_val = self.evaluate_node(left);
                if let Value::Return(_) = left_val {
                    return left_val;
                }
                let right_val = self.evaluate_node(right);
                if let Value::Return(_) = right_val {
                    return right_val;
                }
                match op {
                    TokenKind::Plus => self.evaluate_addition(&left_val, &right_val),
                    TokenKind::Minus => self.evaluate_subtraction(&left_val, &right_val),
                    TokenKind::Star => self.evaluate_multiplication(&left_val, &right_val),
                    TokenKind::Slash => self.evaluate_division(&left_val, &right_val),
                    TokenKind::Equal => Value::Boolean(left_val == right_val),
                    TokenKind::NotEqual => Value::Boolean(left_val != right_val),
                    TokenKind::Greater => {
                        self.evaluate_comparison(&left_val, &right_val, |a, b| a > b)
                    }
                    TokenKind::GreaterEqual => {
                        self.evaluate_comparison(&left_val, &right_val, |a, b| a >= b)
                    }
                    TokenKind::Less => {
                        self.evaluate_comparison(&left_val, &right_val, |a, b| a < b)
                    }
                    TokenKind::LessEqual => {
                        self.evaluate_comparison(&left_val, &right_val, |a, b| a <= b)
                    }
                    TokenKind::BitAnd => self.evaluate_bitwise_and(&left_val, &right_val),
                    TokenKind::BitOr => self.evaluate_bitwise_or(&left_val, &right_val),
                    TokenKind::Assign => match left {
                        ASTNode::Variable(name) => {
                            let right_val = self.evaluate_node(right);
                            if !self.current_scope.borrow_mut().set(name, right_val.clone()) {
                                runtime_error(&format!("Undefined variable: {}", name));
                            }
                            right_val
                        }
                        ASTNode::MemberAccess { object, member } => {
                            let obj_val = self.evaluate_node(object);
                            if let Value::Object(properties) = obj_val {
                                properties
                                    .borrow_mut()
                                    .insert(member.clone(), self.evaluate_node(right));
                                Value::Null
                            } else {
                                runtime_error("Attempted member access on non-object value")
                            }
                        }
                        _ => runtime_error("Left side of assignment must be a variable"),
                    },
                    TokenKind::Mod => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
                        _ => self.bin_op_error(op, &left_val, &right_val),
                    },
                    _ => runtime_error(format!("Unknown binary operator: {:?}", op).as_str()),
                }
            }
        }
    }
    fn evaluate_addition(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(a.clone() + b),
            _ => self.bin_op_error(&TokenKind::Plus, left_val, right_val),
        }
    }

    fn evaluate_subtraction(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => self.bin_op_error(&TokenKind::Minus, left_val, right_val),
        }
    }

    fn evaluate_multiplication(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => self.bin_op_error(&TokenKind::Star, left_val, right_val),
        }
    }

    fn evaluate_division(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => self.bin_op_error(&TokenKind::Slash, left_val, right_val),
        }
    }

    fn evaluate_bitwise_and(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => {
                Value::Number(((*a as i64) & (*b as i64)) as f64)
            }
            _ => self.bin_op_error(&TokenKind::BitAnd, left_val, right_val),
        }
    }

    fn evaluate_bitwise_or(&self, left_val: &Value, right_val: &Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => {
                Value::Number(((*a as i64) & (*b as i64)) as f64)
            }
            _ => self.bin_op_error(&TokenKind::BitAnd, left_val, right_val),
        }
    }

    fn evaluate_comparison<F>(&self, left_val: &Value, right_val: &Value, cmp: F) -> Value
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left_val, right_val) {
            (Value::Number(a), Value::Number(b)) => Value::Boolean(cmp(*a, *b)),
            _ => self.bin_op_error(&TokenKind::Greater, left_val, right_val),
        }
    }
    fn evaluate_unary_op(&mut self, op: &TokenKind, operand: &ASTNode) -> Value {
        let val = self.evaluate_node(operand);
        if let Value::Return(_) = val {
            return val;
        }
        match op {
            TokenKind::Minus => match val {
                Value::Number(n) => Value::Number(-n),
                _ => runtime_error("Operand must be a number"),
            },
            TokenKind::Bang => match val {
                Value::Boolean(b) => Value::Boolean(!b),
                _ => runtime_error("Operand must be a boolean"),
            },
            _ => runtime_error(format!("Unknown unary operator: {:?}", op).as_str()),
        }
    }
}
