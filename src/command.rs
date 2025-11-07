use std::process;

pub const builtin_commands: [&str; 3] = ["exit", "echo", "type"];

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
            "echo" => handle_echo(&self.args),
            "type"=> handle_type(&self.args),
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
fn handle_echo(args: &Vec<String>) {
    let output = args.join(" ");
    println!("{}", output);
}
fn handle_type(args: &Vec<String>) {
    let program = &args[0].as_str();
    if builtin_commands.contains(program) {
        println!("{} is a shell builtin", program);
        return;
    } else {
        match search_in_path(program) {
            Some(path) => println!("{} is {}", program, path),
            None => println!("{}: not found", program),
        }
    }
}
fn search_in_path(program: &str) -> Option<String> {
    // Get the PATH environment variable
    let env_path = std::env::var("PATH").unwrap_or_default();
    let directories = env_path.split(':').collect::<Vec<&str>>();
    for dir in directories {
        let full_path = format!("{}/{}", dir, program);
        // file exists and is executable
        if std::path::Path::new(&full_path).exists() && is_executable(&full_path) {
            return Some(full_path);
        }
    }
    None
}
fn is_executable(path: &str) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(path).ok().unwrap();
        let permissions = metadata.permissions();
        return permissions.mode() & 0o111 != 0; // Check if any execute bit is set
    }
}
