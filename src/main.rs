use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use pitlang::ast::ASTNode;
use pitlang::parser;
use pitlang::tokenizer;
use pitlang::treewalk::evaluator;

fn get_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");

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
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                eprintln!("Error reading input: {}", e);
                continue;
            }
            let tokens = match tokenizer::tokenize(input) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Tokenization error: {}", e.as_message());
                    continue;
                }
            };
            let ast = match parser::parse(tokens.as_slice()) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Parsing error: ");
                    for error in e {
                        eprintln!("{}", error.as_message());
                    }
                    continue;
                }
            };
            if token_arg {
                for token in &tokens {
                    println!("{:?}", token);
                }
            }
            if ast_arg {
                println!("{:?}", ast);
            }
            println!("{:?}", evaluator::evaluate(&ast));
        }
    }

    let file_path = &args[1];
    let contents: String = match get_file_contents(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return;
        }
    };

    let tokens = match tokenizer::tokenize(contents) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Tokenization error: {}", e.as_message());
            return;
        }
    };

    if token_arg {
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    let ast: ASTNode = match parser::parse(tokens.as_slice()) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parsing error: ");
            for error in e {
                eprintln!("{}", error.as_message());
            }
            return;
        }
    };
    if ast_arg {
        println!("{:?}", ast);
    }
    evaluator::evaluate(&ast);
}
