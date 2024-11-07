use crate::ast::ASTNode;
use crate::tokenizer::TokenKind;
use std::collections::HashMap;

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

fn runtime_error(msg: &str) -> Value {
    panic!("Runtime error: {}", msg);
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Function(Vec<String>, ASTNode), // Params and function body
    Null,
    Unit,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => false,
        }
    }
    pub fn print(&self) {
        match self {
            Value::Number(n) => println!("{}", n),
            Value::Boolean(b) => println!("{}", b),
            Value::String(s) => println!("{}", s),
            Value::Function(_, _) => println!("Function"),
            Value::Null => println!("null"),
            Value::Unit => (),
        }
    }
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
        }
        result
    }

    fn evaluate_node(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::NumberLiteral(n) => Value::Number(*n),
            //ASTNode::BooleanLiteral(b) => Value::Boolean(*b),
            ASTNode::StringLiteral(s) => Value::String(s.clone()),
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
            ASTNode::Block(statements) => {
                let mut local_scope = Scope::new(Some(Box::new(self.global_environment.clone())));
                let mut result = Value::Unit;
                for stmt in statements {
                    result = self.evaluate_node_with_scope(stmt, &mut local_scope);
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
                let func = Value::Function(parameters.clone(), *body.clone());
                self.global_environment.insert(name.clone(), func);
                Value::Unit
            }
            ASTNode::FunctionCall { callee, arguments } => {
                let func = self.evaluate_node(callee);

                if let ASTNode::Variable(name) = callee.as_ref() {
                    if name == "print" {
                        let arg = self.evaluate_node(arguments.first().unwrap());
                        arg.print();
                        return Value::Unit;
                    }
                };

                match func {
                    Value::Function(params, body) => {
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
                        result
                    }
                    _ => runtime_error("Called value is not a function"),
                }
            }
            ASTNode::ReturnStatement(expr) => self.evaluate_node(expr),
            _ => runtime_error(format!("Unsupported AST node: {:?}", node).as_str()),
        }
    }

    fn evaluate_node_with_scope(&mut self, node: &ASTNode, scope: &mut Scope) -> Value {
        match node {
            ASTNode::NumberLiteral(n) => Value::Number(*n),
            //ASTNode::BooleanLiteral(b) => Value::Boolean(*b),
            ASTNode::StringLiteral(s) => Value::String(s.clone()),
            ASTNode::Variable(name) => scope
                .get(name)
                .cloned()
                .unwrap_or_else(|| runtime_error(&format!("Undefined variable: {}", name))),
            ASTNode::VariableDeclaration { name, value } => {
                let val = self.evaluate_node_with_scope(value, scope);
                scope.insert(name.clone(), val);
                Value::Unit
            }
            ASTNode::BinaryOp { left, op, right } => self.evaluate_binary_op(op, left, right),
            ASTNode::UnaryOp { op, operand } => self.evaluate_unary_op(op, operand),
            ASTNode::Expression(expr) => self.evaluate_node_with_scope(expr, scope),
            ASTNode::Block(statements) => {
                let mut local_scope = Scope::new(Some(Box::new(scope.clone())));
                let mut result = Value::Unit;
                for stmt in statements {
                    result = self.evaluate_node_with_scope(stmt, &mut local_scope);
                }
                result
            }
            ASTNode::IfStatement {
                condition,
                consequence,
                alternative,
            } => {
                let cond = self.evaluate_node_with_scope(condition, scope);
                match cond {
                    Value::Boolean(true) => self.evaluate_node_with_scope(consequence, scope),
                    Value::Boolean(false) => {
                        if let Some(alt) = alternative {
                            self.evaluate_node_with_scope(alt, scope)
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
                let func = Value::Function(parameters.clone(), *body.clone());
                scope.insert(name.clone(), func);
                Value::Unit
            }
            ASTNode::FunctionCall { callee, arguments } => {
                let func = self.evaluate_node_with_scope(callee, scope);
                match func {
                    Value::Function(params, body) => {
                        if params.len() != arguments.len() {
                            runtime_error("Argument count mismatch");
                        }
                        let mut local_scope = Scope::new(Some(Box::new(scope.clone())));
                        for (param, arg) in params.iter().zip(arguments) {
                            let arg_val = self.evaluate_node_with_scope(arg, scope);
                            local_scope.insert(param.clone(), arg_val);
                        }
                        self.evaluate_node_with_scope(&body, &mut local_scope)
                    }
                    _ => runtime_error("Called value is not a function"),
                }
            }
            ASTNode::ReturnStatement(expr) => self.evaluate_node_with_scope(expr, scope),
            _ => runtime_error(format!("Unsupported AST node: {:?}", node).as_str()),
        }
    }

    fn evaluate_binary_op(&mut self, op: &TokenKind, left: &ASTNode, right: &ASTNode) -> Value {
        let left_val = self.evaluate_node(left);
        let right_val = self.evaluate_node(right);
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
            TokenKind::Assign => {
                if let ASTNode::Variable(name) = left {
                    self.global_environment
                        .insert(name.clone(), right_val.clone());
                    right_val
                } else {
                    runtime_error("Left side of assignment must be a variable")
                }
            }
            _ => runtime_error(format!("Unknown binary operator: {:?}", op).as_str()),
        }
    }

    fn evaluate_unary_op(&mut self, op: &TokenKind, operand: &ASTNode) -> Value {
        let val = self.evaluate_node(operand);
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
