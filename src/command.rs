use std::os::unix::process::CommandExt;
use std::process; // <-- needed

pub const builtin_commands: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

#[derive(Debug)]
pub struct Command {
    program: String,   // The name of the command/program
    args: Vec<String>, // The arguments passed to that program
}

impl From<&str> for Command {
    // support single quotes and double quotes in arguments
    fn from(value: &str) -> Self {
        let mut parts = vec![];
        let mut current = String::new();
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;

        let mut chars = value.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\'' if !in_double_quotes => {
                    in_single_quotes = !in_single_quotes;
                    continue;
                }
                '"' if !in_single_quotes => {
                    in_double_quotes = !in_double_quotes;
                    continue;
                }
                ' ' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                    continue;
                }

                '\\' if !in_single_quotes && !in_double_quotes => {
                    // Take next char literally if exists
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                    continue;
                }
                '\\' if in_double_quotes => {
                    // In double quotes, only certain characters can be escaped
                    if let Some(&next) = chars.peek() {
                        if next == '"' || next == '\\' || next == '$' || next == '`' {
                            chars.next(); // consume the next character
                            current.push(next);
                            continue;
                        }
                    }
                    current.push(c); // treat backslash literally
                    continue;
                }
                _ => {}
            }
            current.push(c);
        }
        if !current.is_empty() {
            parts.push(current);
        }

        let program = parts.get(0).cloned().unwrap_or_default();
        let args = if parts.len() > 1 {
            parts[1..].to_vec()
        } else {
            vec![]
        };

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
    let target_dir = if args.is_empty() || args[0] == "~" {
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
