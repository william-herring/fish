use std::fs;
use std::io;
use std::io::{BufRead, Write};

pub struct Interpreter;

impl Interpreter {
    pub fn execute(source: String) {
        // test
        println!("{source}");
    }

    pub fn execute_file(path_to_file: String) -> Result<(), io::Error> {
        let contents = fs::read_to_string(path_to_file)?;
        Self::execute(contents);

        Ok(())
    }

    pub fn shell() -> Result<(), io::Error> {
        let instream = io::stdin();
        print!(">> ");
        io::stdout().flush().expect("Prompt could not be output.");

        for line in instream.lock().lines() {
            match line {
                Ok(input) => {
                    if input != "" {
                        Self::execute(input);
                    }
                }
                Err(e) => {
                    Self::error(-1, format!("Could not read line {}", e));
                }
            }
            print!(">> ");
            io::stdout().flush().expect("Prompt could not be output.");
        }
        println!("Exited shell");

        Ok(())
    }

    pub fn error(line: i32, msg: String) {
        eprintln!("[Line {}] Error: {}", line, msg);
    }
}