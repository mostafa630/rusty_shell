#[allow(unused_imports)]
use std::io::{self, Write};

mod command;
use command::Command;
fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to take input");

        let command: Command = command.trim().into();
        command.execute();
    }
}

///////////////////////// code to take it as refrence /////////////////////////
// Represents a parsed command (no piping)
// pub struct Command {
//     pub program: String,
//     pub args: Vec<String>,
// }

// // Represents a full shell line which may contain pipes
// pub struct Pipeline {
//     pub commands: Vec<Command>,  // e.g. ["ls -l", "grep txt"]
// }

// // Execution environment (shell state)
// pub struct Shell {
//     pub current_dir: std::path::PathBuf,
//     pub last_status: i32,
// }

// // How the command should run
// pub enum ExecMode {
//     Foreground,
//     Background, // with '&'
// }

// // Where to redirect input/output
// pub enum Redirection {
//     Input(String),           // < file
//     OutputTruncate(String),  // > file
//     OutputAppend(String),    // >> file
// }

// // Token types used during parsing
// pub enum Token {
//     Word(String),
//     Pipe,      // |
//     Ampersand, // &
//     RedirectIn,      // <
//     RedirectOut,     // >
//     RedirectAppend,  // >>
// }
// // Result of parsing a single line
// pub struct ParsedLine {
//     pub pipeline: Pipeline,
//     pub mode: ExecMode,
//     pub redirections: Vec<Redirection>,
// }
// These give you:

// Tokenizer → produces Vec<Token>

// Parser → converts tokens to ParsedLine

// Executor → takes ParsedLine
// Shell → stores state (pwd, last exit code, env if needed)
