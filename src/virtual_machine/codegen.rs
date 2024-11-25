use crate::ast::ASTNode;
use crate::tokenizer::TokenKind;
use crate::virtual_machine::bytecode::{Bytecode, OpCode};
use crate::virtual_machine::value::{Value, ValueType};
use std::collections::HashMap;

#[derive(Default)]
pub struct Compiler {
    pub globals: HashMap<String, u16>, // Maps variable names to constant indices
    pub locals: Vec<HashMap<String, u16>>, // Stack of local scopes
    pub bytecode: Bytecode,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            locals: Vec::new(),
            bytecode: Bytecode::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.locals.pop();
    }

    pub fn add_global(&mut self, name: String) -> Result<u16, String> {
        if self.globals.contains_key(&name) {
            // Variable already exists
            return Err(format!("Global variable '{}' already defined", name));
        }
        let variable_index = self
            .bytecode
            .add_constant(Value::new_object::<String>(name.clone()))?;
        self.globals.insert(name, variable_index);
        Ok(variable_index)
    }
}

pub fn compile_ast(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    match node {
        ASTNode::Block(statements) => {
            for statement in statements {
                compile_ast(compiler, statement)?;
            }
            Ok(())
        }
        ASTNode::NumberLiteral(_)
        | ASTNode::StringLiteral(_)
        | ASTNode::BooleanLiteral(_)
        | ASTNode::NullLiteral => compile_literal(compiler, node),
        ASTNode::BinaryOp { .. } => compile_binary_op(compiler, node),
        ASTNode::VariableDeclaration { .. } => compile_variable_declaration(compiler, node),
        ASTNode::IfStatement { .. } => compile_if_statement(compiler, node),
        ASTNode::FunctionDeclaration { .. } => compile_function_declaration(compiler, node),
        ASTNode::ReturnStatement(expr) => {
            compile_ast(compiler, *expr)?;
            compiler.bytecode.push_op(OpCode::RETURN);
            Ok(())
        }
        ASTNode::Variable(name) => compile_variable(compiler, name),
        ASTNode::WhileStatement { .. } => compile_while_statement(compiler, node),
        _ => Err("Unsupported AST node".to_string()),
    }
}

fn compile_literal(compiler: &mut Compiler, value: ASTNode) -> Result<(), String> {
    let constant_index = match value {
        ASTNode::NumberLiteral(num) => compiler.bytecode.add_constant(Value::new_float(num))?,
        ASTNode::StringLiteral(s) => compiler
            .bytecode
            .add_constant(Value::new_object::<String>(s))?,
        ASTNode::BooleanLiteral(b) => compiler.bytecode.add_constant(Value::new_boolean(b))?,
        ASTNode::NullLiteral => compiler.bytecode.add_constant(Value::new_null())?,
        _ => return Err("Invalid literal".to_string()),
    };
    compiler.bytecode.push_op(OpCode::CONST(constant_index));
    Ok(())
}

fn compile_variable(compiler: &mut Compiler, name: String) -> Result<(), String> {
    // Try to resolve the variable in local scopes first
    for scope in compiler.locals.iter().rev() {
        if let Some(&index) = scope.get(&name) {
            compiler.bytecode.push_op(OpCode::LOAD_LOCAL(index));
            return Ok(());
        }
    }

    // If not found in locals, check globals
    if let Some(&index) = compiler.globals.get(&name) {
        compiler.bytecode.push_op(OpCode::LOAD_GLOBAL(index));
        return Ok(());
    }

    // Variable not found
    Err(format!("Undefined variable '{}'", name))
}

fn compile_binary_op(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    if let ASTNode::BinaryOp { left, op, right } = node {
        if op == TokenKind::Assign {
            // Special case
            if let ASTNode::Variable(name) = *left {
                compile_ast(compiler, *right)?;
                if !compiler.locals.is_empty() {
                    // Add to the current local scope
                    let local_scope = compiler.locals.last_mut().unwrap();
                    if let Some(&index) = local_scope.get(&name) {
                        compiler.bytecode.push_op(OpCode::STORE_LOCAL(index));
                    } else {
                        return Err(format!("Undefined variable '{}'", name));
                    }
                } else {
                    // Global scope
                    if let Some(&index) = compiler.globals.get(&name) {
                        compiler.bytecode.push_op(OpCode::STORE_GLOBAL(index));
                    } else {
                        return Err(format!("Undefined variable '{}'", name));
                    }
                }
                return Ok(());
            }
        }

        // Compile the left and right operands
        compile_ast(compiler, *left)?;
        compile_ast(compiler, *right)?;

        // Emit the operation opcode
        let opcode = match op {
            TokenKind::Plus => OpCode::ADD,
            TokenKind::Minus => OpCode::SUB,
            TokenKind::Star => OpCode::MUL,
            TokenKind::Slash => OpCode::DIV,
            TokenKind::Equal => OpCode::EQ,
            TokenKind::NotEqual => OpCode::NEQ,
            TokenKind::Less => OpCode::LT,
            TokenKind::LessEqual => OpCode::LTE,
            TokenKind::Greater => OpCode::GT,
            TokenKind::GreaterEqual => OpCode::GTE,
            _ => return Err("Unsupported binary operator".to_string()),
        };
        compiler.bytecode.push_op(opcode);
    }
    Ok(())
}

fn compile_if_statement(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    if let ASTNode::IfStatement {
        condition,
        consequence,
        alternative,
    } = node
    {
        // Compile the condition
        compile_ast(compiler, *condition)?;

        // Emit a conditional jump (placeholder address)
        let jump_if_false_addr = compiler.bytecode.code.len();
        compiler.bytecode.push_op(OpCode::JUMP_IF_FALSE(0));

        // Compile the consequence
        compile_ast(compiler, *consequence)?;

        // Emit an unconditional jump to skip the alternative
        let jump_addr = compiler.bytecode.code.len();
        compiler.bytecode.push_op(OpCode::JUMP(0));

        // Patch the jump_if_false address
        let code_len = compiler.bytecode.code.len();
        if let OpCode::JUMP_IF_FALSE(ref mut addr) = compiler.bytecode.code[jump_if_false_addr] {
            *addr = code_len;
        }

        // Compile the alternative, if present
        if let Some(alt) = alternative {
            compile_ast(compiler, *alt)?;
        }

        let code_len = compiler.bytecode.code.len();
        // Patch the unconditional jump address
        if let OpCode::JUMP(ref mut addr) = compiler.bytecode.code[jump_addr] {
            *addr = code_len;
        }
    }
    Ok(())
}

fn compile_function_declaration(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    if let ASTNode::FunctionDeclaration {
        name,
        parameters,
        body,
    } = node
    {
        // Create a new compiler for the function
        let mut function_compiler = Compiler::new();

        // Push a new local scope
        function_compiler.push_scope();

        // Add parameters to the local scope
        for (i, param) in parameters.iter().enumerate() {
            function_compiler
                .locals
                .last_mut()
                .unwrap()
                .insert(param.clone(), i as u16);
        }

        // Compile the function body
        compile_ast(&mut function_compiler, *body)?;

        // Pop the local scope
        function_compiler.pop_scope();

        // Create a function value
        let function_value = Value::new_function(parameters.clone(), function_compiler.bytecode);
        let constant_index = compiler.bytecode.add_constant(function_value)?;

        // Store the function in the global scope
        if let Some(func_name) = name {
            let variable_index = compiler.add_global(func_name.clone())?;
            compiler.bytecode.push_op(OpCode::CONST(constant_index));
            compiler
                .bytecode
                .push_op(OpCode::STORE_GLOBAL(variable_index));
        }
    }
    Ok(())
}

fn compile_variable_declaration(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    if let ASTNode::VariableDeclaration { name, value } = node {
        // Compile the value expression
        compile_ast(compiler, *value)?;

        // Check if we're in a local scope
        if !compiler.locals.is_empty() {
            // Add to the current local scope
            let local_scope = compiler.locals.last_mut().unwrap();
            let variable_index = local_scope.len() as u16;
            local_scope.insert(name.clone(), variable_index);
            compiler
                .bytecode
                .push_op(OpCode::STORE_LOCAL(variable_index));
        } else {
            // Global scope
            let variable_index = compiler.add_global(name.clone())?;
            compiler
                .bytecode
                .push_op(OpCode::STORE_GLOBAL(variable_index));
        }
    }
    Ok(())
}

fn compile_while_statement(compiler: &mut Compiler, node: ASTNode) -> Result<(), String> {
    if let ASTNode::WhileStatement { condition, body } = node {
        let unconditional_jump_pos = compiler.bytecode.code.len();

        // Compile the condition
        compile_ast(compiler, *condition)?;

        // Emit a conditional jump (placeholder address)
        let jump_if_false_addr = compiler.bytecode.code.len();
        compiler.bytecode.push_op(OpCode::JUMP_IF_FALSE(0));

        // Compile the body
        compile_ast(compiler, *body)?;

        // Emit an unconditional jump back to the condition
        let jump_addr = compiler.bytecode.code.len();
        compiler.bytecode.push_op(OpCode::JUMP(0));

        // Patch the jump_if_false address
        let code_len = compiler.bytecode.code.len();
        if let OpCode::JUMP_IF_FALSE(ref mut addr) = compiler.bytecode.code[jump_if_false_addr] {
            *addr = code_len;
        }

        // Patch the unconditional jump address
        if let OpCode::JUMP(ref mut addr) = compiler.bytecode.code[jump_addr] {
            *addr = unconditional_jump_pos;
        }
        return Ok(());
    }
    Err("Invalid while statement".to_string())
}
