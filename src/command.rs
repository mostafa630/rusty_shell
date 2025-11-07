use std::os::unix::process::CommandExt;
use std::process; // <-- needed

pub const builtin_commands: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

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
            "exit" => run_exit(&self.args),
            "echo" => run_echo(&self.args),
            "type" => run_type(&self.args),
            "pwd" => run_pwd(),
            "cd" => run_cd(&self.args),
            _ => {
                if is_external_program(&self.program) {
                    run_external_programs(&self.program, &self.args);
                } else {
                    println!("{}: command not found", self.program);
                }
            }
        }
    }
}
fn run_exit(args: &Vec<String>) {
    let exit_code_str = &args[0];
    if let Ok(exit_code) = exit_code_str.parse::<i32>() {
        process::exit(exit_code);
    }
    process::exit(1);
}
fn run_echo(args: &Vec<String>) {
    let output = args.join(" ");
    println!("{}", output);
}
fn run_type(args: &Vec<String>) {
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
fn run_pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }
}
fn run_cd(args: &Vec<String>) {
    let target_dir = if args.is_empty() {
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };
    if let Err(e) = std::env::set_current_dir(&target_dir) {
        println!("cd: {}: {}", target_dir, "No such file or directory");
    }
}

fn run_external_programs(program: &str, args: &Vec<String>) {
    match search_in_path(program) {
        Some(path) => {
            let mut child = process::Command::new(path)
                .arg0(program)
                .args(args)
                .spawn()
                .expect("failed to execute process");
            child.wait().expect("failed to wait on child");
        }
        None => println!("{}: command not found", program),
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
fn is_external_program(program: &str) -> bool {
    match search_in_path(program) {
        Some(_) => true,
        None => false,
    }
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
