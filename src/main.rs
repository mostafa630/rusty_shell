#[allow(unused_imports)]
use std::io::{self, Write};
use rustyline::Editor;
use rustyline::error::ReadlineError;

use rusty_shell::auto_complete::AutoCompleter;
use rusty_shell::executor::Executor;
use rusty_shell::parser::Parser;
use rusty_shell::tokenizer::Tokenizer;
use rustyline::history::DefaultHistory;

// 
fn main() {
    // Create rustyline editor with our completer
    let mut rl= Editor::<AutoCompleter ,DefaultHistory>::new().unwrap();
    rl.set_helper(Some(AutoCompleter::new()));

    loop {
        // Read input line with prompt
        let readline = rl.readline("$");
        match readline {
            Ok(input_line) => {
                let input_line = input_line.trim();
                if input_line.is_empty() {
                    continue;
                }

                // Add to history so you can navigate with up/down arrows
                rl.add_history_entry(input_line);

                // Tokenize, parse, and execute using your existing shell logic
                let tokens = Tokenizer::tokenize(input_line);
                let parsed_line = Parser::parse(tokens);

                // Execute your command
                Executor::execute(parsed_line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
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
