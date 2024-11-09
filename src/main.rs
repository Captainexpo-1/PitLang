use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;

use pitlang::common::Function;
use pitlang::parser;
use pitlang::tokenizer;
use pitlang::treewalk::evaluator;
use pitlang::virtualmachine::bytecode::dump_bytecode;
use pitlang::virtualmachine::codegen::CodeGenerator;
use pitlang::virtualmachine::virtualmachine::VM;

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
        let mut codegen = CodeGenerator::new();
        let bytecode = codegen.generate_bytecode(&ast);
        if args.contains(&String::from("-d")) {
            dump_bytecode(&bytecode.0, &bytecode.1, "bytecode.txt");
        }
        // Create a VM instance
        let mut vm = VM::new();

        let main_function = Rc::new(Function {
            instructions: bytecode.0,
            constants: bytecode.1,
            parameters: vec![],
            name: Some("main".to_string()),
        });

        // Run the main function in the VM
        let result = vm.run(main_function);
        println!("{:?}", result);
    }
    //println!("{}", result);
}
