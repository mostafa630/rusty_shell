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
        
        handle_exit(&mut command);
        println!("{}: command not found", command.trim());
    }
}
fn handle_exit(command: &mut String) {
    let mut parts = command.split_whitespace();
    let args : Vec<String> = parts.map(|s| s.to_string()).collect();
    if args.len()== 2 && args[0] == "exit" {
        let exit_code_str = &args[1];
        if let Ok(exit_code) = exit_code_str.parse::<i32>() {
            process::exit(exit_code);
        } 
        process::exit(1);
    }
}
