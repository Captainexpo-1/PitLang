use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::rc::Rc;

use pitlang::parser;
use pitlang::tokenizer;
use pitlang::treewalk::evaluator;

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

    if args.contains(&String::from("-h")) {
        println!("Usage: {} <file> [-t] [-ast] [-eval]", args[0]);
        println!("\t-t: Tokenize only");
        println!("\t-ast: Print AST");
        println!("\t-eval: Evaluate AST");
        return;
    }

    if args.contains(&String::from("-repl")) {
        loop {
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            let tokens = tokenizer::tokenize(input);
            let ast = parser::parse(tokens.as_slice());
            evaluator::evaluate(&ast);
        }
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

    //println!("{}", result);
}
