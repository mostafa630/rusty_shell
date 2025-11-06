use std::process;

#[derive(Debug)]
pub struct Command {
    program: String,   // The name of the command/program
    args: Vec<String>, // The arguments passed to that program
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let program = parts.next().unwrap_or("").to_string();
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
        Command { program, args }
    }
}

impl Command {
    pub fn execute(&self) {
        match self.program.as_str() {
            "exit" => handle_exit(&self.args),
            "echo" => handle_ecoh(&self.args),
            _ => {
                println!("{}: command not found", self.program);
            }
        }
    }
}
fn handle_exit(args: &Vec<String>) {
    let exit_code_str = &args[0];
    if let Ok(exit_code) = exit_code_str.parse::<i32>() {
        process::exit(exit_code);
    }
    process::exit(1);
}
fn handle_ecoh(args: &Vec<String>) {
    let output = args.join(" ");
    println!("{}", output);
}
