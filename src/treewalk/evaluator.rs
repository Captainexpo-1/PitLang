use crate::ast::ASTNode;
use crate::common::Value;
use crate::stdlib::{array_methods, number_methods, string_methods};
use crate::tokenizer::TokenKind;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

#[derive(Clone)]
struct Scope {
    variables: HashMap<String, Value>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Box<Scope>>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent,
        }
    }
    pub fn insert(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        match self.variables.get(name) {
            Some(val) => Some(val),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }
}

struct TreeWalk<'a> {
    program: &'a Vec<ASTNode>,
    global_environment: Scope,
}

impl<'a> TreeWalk<'a> {
    pub fn new(program: &'a Vec<ASTNode>) -> Self {
        TreeWalk {
            program,
            global_environment: Scope::new(None),
        }
    }

    fn evaluate_program(&mut self) -> Value {
        self.global_environment
            .insert("print".to_string(), Value::Null);

        let mut result = Value::Unit;
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
                .global_environment
                .get(name)
                .cloned()
                .unwrap_or_else(|| runtime_error(&format!("Undefined variable: {}", name))),
            ASTNode::VariableDeclaration { name, value } => {
                let val = self.evaluate_node(value);
                self.global_environment.insert(name.clone(), val);
                Value::Unit
            }
            ASTNode::Expression(expr) => self.evaluate_node(expr),
            ASTNode::BinaryOp { left, op, right } => self.evaluate_binary_op(op, left, right),
            ASTNode::UnaryOp { op, operand } => self.evaluate_unary_op(op, operand),
            ASTNode::MemberAccess { object, member } => {
                let obj_val = self.evaluate_node(object);
                if let Value::Object(properties) = obj_val {
                    properties.borrow().get(member).cloned().unwrap_or_else(|| {
                        runtime_error(&format!("Property '{}' not found", member))
                    })
                } else {
                    Value::Method {
                        receiver: Box::new(obj_val),
                        method_name: member.clone(),
                    }
                }
            }
            ASTNode::Block(statements) => {
                let mut result = Value::Unit;
                for stmt in statements {
                    result = self.evaluate_node(stmt);
                    if let Value::Return(_) = result {
                        break;
                    }
                }
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
                            Value::Unit
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
                let func = Value::Function_dep(parameters.clone(), *body.clone());

                if name.is_some() {
                    self.global_environment.insert(name.clone().unwrap(), func);
                    return Value::Unit;
                }
                func
            }
            ASTNode::WhileStatement { condition, body } => {
                let mut result = Value::Unit;
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

                if let ASTNode::Variable(name) = callee.as_ref() {
                    if name == "print" {
                        let arg = self.evaluate_node(arguments.first().unwrap());
                        arg.print();
                        println!();
                        return Value::Unit;
                    }
                };

                match func {
                    Value::Function_dep(params, body) => {
                        if params.len() != arguments.len() {
                            runtime_error("Argument count mismatch");
                        }
                        let mut local_scope =
                            Scope::new(Some(Box::new(self.global_environment.clone())));
                        for (param, arg) in params.iter().zip(arguments) {
                            let arg_val = self.evaluate_node(arg);
                            local_scope.insert(param.clone(), arg_val);
                        }
                        let previous_env =
                            std::mem::replace(&mut self.global_environment, local_scope);
                        let result = self.evaluate_node(&body);
                        self.global_environment = previous_env;
                        if let Value::Return(val) = result {
                            *val
                        } else {
                            result
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

        let string_methods = string_methods();
        let number_methods = number_methods();
        let array_methods = array_methods();
        let method = match &receiver {
            Value::String(_) => string_methods.get(method_name),
            Value::Number(_) => number_methods.get(method_name),
            Value::Array(_) => array_methods.get(method_name),
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
    fn evaluate_binary_op(&mut self, op: &TokenKind, left: &ASTNode, right: &ASTNode) -> Value {
        let left_val = self.evaluate_node(left);
        if let Value::Return(_) = left_val {
            return left_val;
        }
        let right_val = self.evaluate_node(right);
        if let Value::Return(_) = right_val {
            return right_val;
        }
        match op {
            TokenKind::Plus => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                (Value::String(a), Value::String(b)) => Value::String(a + &b),
                _ => runtime_error("Operands must be two numbers or two strings"),
            },
            TokenKind::Minus => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::Star => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::Slash => match (left_val, right_val) {
                (Value::Number(_), Value::Number(0.0)) => runtime_error("Division by zero"),
                (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::Equal => Value::Boolean(left_val == right_val),
            TokenKind::NotEqual => Value::Boolean(left_val != right_val),
            TokenKind::Greater => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::GreaterEqual => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Boolean(a >= b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::LessEqual => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Boolean(a <= b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::Less => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Boolean(a < b),
                _ => runtime_error("Operands must be numbers"),
            },
            TokenKind::Assign => match left {
                ASTNode::Variable(name) => {
                    self.global_environment
                        .insert(name.clone(), right_val.clone());
                    right_val
                }
                ASTNode::MemberAccess { object, member } => {
                    let obj_val = self.evaluate_node(object);
                    if let Value::Object(properties) = obj_val {
                        properties
                            .borrow_mut()
                            .insert(member.clone(), right_val.clone());
                        Value::Object(properties)
                    } else {
                        runtime_error("Attempted member access on non-object value")
                    }
                }
                _ => runtime_error("Left side of assignment must be a variable"),
            },
            TokenKind::Mod => match (left_val, right_val) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
                _ => runtime_error("Operands must be numbers"),
            },
            _ => runtime_error(format!("Unknown binary operator: {:?}", op).as_str()),
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
