use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

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

    let ast_arg = args.contains(&String::from("-ast"));
    let token_arg = args.contains(&String::from("-t"));

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
            if ast_arg {
                println!("{:?}", ast);
            }
            println!("{:?}", evaluator::evaluate(&ast));
        }
    }

    let file_path = &args[1];
    let contents = get_file_contents(file_path);
    let tokens = tokenizer::tokenize(contents);

    if token_arg {
        for token in &tokens {
            println!("{:?}", token);
        }
        return;
    }

    let ast = parser::parse(tokens.as_slice());
    if ast_arg {
        println!("{:?}", ast);
    }
    evaluator::evaluate(&ast);
}
