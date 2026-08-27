use std::env;
mod interpreter;
mod lexer;
pub mod parser;

use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: fish <file_path>");
    } else if args.len() == 2 {
        let file_path = &args[1];
        let res = Interpreter::execute_file(file_path.clone());
        if res.is_err() {
            Interpreter::error(-1, format!("Could not extract program at {}", file_path));
        }
    } else {
        Interpreter::shell().expect("Shell terminated unexpectedly");
    }
}
