use std::env;
mod interpreter;
use interpreter::Interpreter;


fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() > 1) {
        println!("Usage: fish <filename>");
    } else if (args.len() == 1) {
        Interpreter::execute(args[0].clone());
    } else {
        Interpreter::shell();
    }
}
