use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;

use pitlang::parser;
use pitlang::tokenizer;
use pitlang::treewalk::evaluator;
use pitlang::virtualmachine::bytecode::Instruction;
use pitlang::virtualmachine::bytecode::{dump_bytecode, Bytecode};
use pitlang::virtualmachine::codegen::CodeGenerator;
use pitlang::virtualmachine::interpreter::Interpreter;
use pitlang::virtualmachine::value::Value;

fn get_file_contents(file_path: &str) -> String {
    let file = File::open(file_path).expect("File not found");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Error reading file");
    contents
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path = &args[1];
    let contents = get_file_contents(file_path);
    let tokens = tokenizer::tokenize(contents);

    if args.contains(&String::from("-t")) {
        for token in &tokens {
            println!("{:?}", token);
        }
        return;
    }

    let ast = parser::parse(tokens.as_slice());
    if args.contains(&String::from("-ast")) {
        println!("{:?}", ast);
    }
    if args.contains(&String::from("-eval")) {
        evaluator::evaluate(&ast);
    }
    if args.contains(&String::from("-vm")) {
        let instructions = vec![
            Instruction::PushConst(0),
            Instruction::PushConst(1),
            Instruction::Add,
            Instruction::PushConst(2),
            Instruction::Sub,
            Instruction::PushConst(3),
            Instruction::Mul,
            Instruction::PushConst(4),
            Instruction::Div,
            Instruction::Halt,
        ];
        let constants = vec![
            Value::Number(10.0),
            Value::Number(5.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(2.0),
        ];

        let bytecode = Bytecode {
            instructions,
            constants,
        };

        dump_bytecode(&bytecode, "../output/bytecode.txt");

        let mut interpreter = Interpreter::new();
        interpreter.run(bytecode);

        interpreter.dump_stack();
    }
    //println!("{}", result);
}
