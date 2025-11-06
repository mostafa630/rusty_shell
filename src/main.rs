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
