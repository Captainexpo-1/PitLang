use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

//use strungs::evaluator;
use strungs::evaluator;
use strungs::parser;
use strungs::tokenizer;

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
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path = &args[1];
    let contents = get_file_contents(file_path);
    let tokens = tokenizer::tokenize(contents);

    for token in &tokens {
        println!("{:?}", token);
    }

    let ast = parser::parse(tokens.as_slice());
    println!("{:?}", ast);
    evaluator::evaluate(&ast);
    //println!("{}", result);
}
