mod lexer;
mod parser;
mod compiler;

use std::env;
use std::fs;
use compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ripc <input_file>.rip");
        return;
    }

    let input_file = &args[1];

    let source_code = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let compiler = Compiler::new("1.0.0"); // Sürümü burada belirtiyorum
    let result = compiler.compile_file(&source_code);

    println!("Compiler result: {}", result);
}
