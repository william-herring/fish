use std::fs;
use std::io;
use std::io::BufRead;

pub struct Interpreter;

impl Interpreter {
    pub fn execute(source: String) {
        
    }

    pub fn execute_file(path_to_file: String) -> Result<(), io::Error> {
        let contents = fs::read_to_string(path_to_file)?;
        Self::execute(contents);

        Ok(())
    }

    pub fn shell() -> Result<(), io::Error> {
        let instream = io::stdin();
        print!(">> ");

        for line in instream.lock().lines() {
            match line {
                Ok(input) => {
                    Self::execute(input);
                }
                Err(e) => {
                    Self::error(-1, format!("Could not read line {}", e));
                }
            }
        }
        println!("Exited shell");

        Ok(())
    }

    pub fn error(line: i32, msg: String) {
        eprintln!("[Line {}] Error: {}", line, msg);
    }
}