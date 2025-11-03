#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command  = String::new();
    io::stdin().read_line(&mut command).expect("failed to take input");
    println!("{}: command not found", command.trim());
    
}
