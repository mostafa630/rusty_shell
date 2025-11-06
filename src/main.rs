#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to take input");
        if command.trim() == "exit" {
            process::exit(0);
        }
        println!("{}: command not found", command.trim());
    }
}
