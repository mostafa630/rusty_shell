use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process;

pub const builtin_commands: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

#[derive(Debug)]
pub struct CommandOutput {
    pub success: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum RedirectCode {
    One(String),
    Two(String),
}

#[derive(Debug)]
pub enum Redirection {
    Input(String),                // < file
    OutputTruncate(RedirectCode), // > file   (remove existing content and add new content)
    OutputAppend(String),         // >> file  (append new content to existing content)
}
#[derive(Debug)]
pub struct Command {
    pub program: String,   // The name of the command/program
    pub args: Vec<String>, // The arguments passed to that program
    pub redirection: Option<Redirection>,
}

impl Command {
    pub fn execute(&self) -> CommandOutput {
        match self.program.as_str() {
            "exit" => run_exit(&self.args),
            "echo" => run_echo(&self.args),
            "type" => run_type(&self.args),
            "pwd" => run_pwd(),
            "cd" => run_cd(&self.args),
            _ => {
                if is_external_program(&self.program) {
                    run_external_programs(&self.program, &self.args)
                } else {
                    CommandOutput {
                        success: None,
                        error: Some(format!("{}: command not found\n", self.program)),
                    }
                }
            }
        }
    }
}
fn run_exit(args: &Vec<String>) -> CommandOutput {
    let exit_code_str = &args[0];
    if let Ok(exit_code) = exit_code_str.parse::<i32>() {
        process::exit(exit_code);
    }
    process::exit(1);
}
fn run_echo(args: &Vec<String>) -> CommandOutput {
    CommandOutput {
        success: Some(format!("{}\n", args.join(" "))),
        error: None,
    }
}
fn run_type(args: &Vec<String>) -> CommandOutput {
    let program = &args[0].as_str();
    get_type(program)
}
fn get_type(program: &str) -> CommandOutput {
    if builtin_commands.contains(&program) {
        CommandOutput {
            success: Some(format!("{} is a shell builtin\n", program)),
            error: None,
        }
    } else {
        match search_in_path(program) {
            Some(path) => CommandOutput {
                success: Some(format!("{} is {}\n", program, path)),
                error: None,
            },
            None => CommandOutput {
                success: None,
                error: Some(format!("{}: not found\n", program)),
            },
        }
    }
}
fn run_pwd() -> CommandOutput {
    match std::env::current_dir() {
        Ok(path) => CommandOutput {
            success: Some(format!("{}\n", path.display())),
            error: None,
        },
        Err(e) => CommandOutput {
            success: None,
            error: Some(format!("Error getting current directory: {}\n", e)),
        },
    }
}

fn run_cd(args: &Vec<String>) -> CommandOutput {
    let target_dir = if args.is_empty() || args[0] == "~" {
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };

    if let Err(_) = std::env::set_current_dir(&target_dir) {
        return CommandOutput {
            success: None,
            error: Some(format!("cd: {}: No such file or directory\n", target_dir)),
        };
    }

    CommandOutput {
        success: None, // cd usually doesn't output anything on success
        error: None,
    }
}

fn run_external_programs(program: &str, args: &Vec<String>) -> CommandOutput {
    get_external_program_output(program, args)
}
fn get_external_program_output(program: &str, args: &Vec<String>) -> CommandOutput {
    match search_in_path(program) {
        Some(path) => {
            let output = process::Command::new(path)
                .arg0(program)
                .args(args)
                .output()
                .expect("failed to execute process");

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            CommandOutput {
                success: if stdout.is_empty() {
                    None
                } else {
                    Some(stdout)
                },
                error: if stderr.is_empty() {
                    None
                } else {
                    Some(stderr)
                },
            }
        }
        None => CommandOutput {
            success: None,
            error: Some(format!("{}: command not found\n", program)),
        },
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

fn needs_newline(output: &str) -> bool {
    output
        .chars()
        .any(|c| !c.is_control() && !c.is_whitespace())
}
