#[allow(unused_imports)]
use std::io::{self, Write};

mod command;
mod executor;
mod parser;
mod tokenizer;
use command::Command;

use crate::executor::Executor;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("failed to take input");

        let tokens = Tokenizer::tokenize(&input_line.trim());
        let parsed_line = Parser::parse(tokens);
        //println!("{:?}", parsed_line);
        Executor::execute(parsed_line);
    }
}

// // Execution environment (shell state)
// pub struct Shell {
//     pub current_dir: std::path::PathBuf,
//     pub last_status: i32,
// }

// Tokenizer → produces Vec<Token>

// Parser → converts tokens to ParsedLine

// Executor → takes ParsedLine
// Shell → stores state (pwd, last exit code, env if needed)
