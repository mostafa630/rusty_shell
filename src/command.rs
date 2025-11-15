use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process;

pub const builtin_commands: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];


#[derive(Debug)]
pub enum OutputType {
    Success(Option<String>),
    Error(Option<String>)
}

#[derive(Debug)]
pub enum RedirectCode{
    One(String), 
    Two(String)
}

#[derive(Debug)]
pub enum Redirection {
    Input(String),          // < file
    OutputTruncate(RedirectCode), // > file   (remove existing content and add new content)
    OutputAppend(String),   // >> file  (append new content to existing content)
}
#[derive(Debug)]
pub struct Command {
    pub program: String,   // The name of the command/program
    pub args: Vec<String>, // The arguments passed to that program
    pub redirection: Option<Redirection>,
}

impl Command {
    pub fn execute(&self)-> OutputType {
        match self.program.as_str() {
            "exit" => run_exit(&self.args),
            "echo" => run_echo(&self.args, &self.redirection),
            "type" => run_type(&self.args, &self.redirection),
            "pwd" => run_pwd(&self.redirection),
            "cd" => run_cd(&self.args, &self.redirection),
            _ => {
                if is_external_program(&self.program) {
                    run_external_programs(&self.program, &self.args, &self.redirection)
                } else {
                    // let error_message = format!("{}: command not found", self.program);
                    // match &self.redirection {
                    //     Some(file) => redirect(file, &error_message),
                    //     None => {
                    //         println!("{}", error_message)
                    //     }
                    // }
                    let error_message = format!("{}: command not found\n", self.program);
                    OutputType::Error(Some(error_message))

                }
            }
        }
    }
}
fn run_exit(args: &Vec<String>)->OutputType {
    let exit_code_str = &args[0];
    if let Ok(exit_code) = exit_code_str.parse::<i32>() {
        process::exit(exit_code);
    }
    process::exit(1);
}
fn run_echo(args: &Vec<String>, redirection: &Option<Redirection>)-> OutputType {
    // let output = args.join(" ");
    // match redirection {
    //     Some(file) => redirect(file, &output),
    //     None => {
    //         println!("{}", output)
    //     }
    // }
    OutputType::Success(Some(format!("{}\n" ,args.join(" ") )))
}
fn run_type(args: &Vec<String>, redirection: &Option<Redirection>)->OutputType {
    // let program = &args[0].as_str();
    // let program_type = get_type(program);
    // match redirection {
    //     Some(file) => redirect(file, &program_type),
    //     None => {
    //         println!("{}", program_type)
    //     }
    // }
     let program = &args[0].as_str();
     get_type(program)

}
fn get_type(program: &str) -> OutputType {
    if builtin_commands.contains(&program) {
        return OutputType::Success(Some(format!("{} is a shell builtin\n", program)))
    } else {
        match search_in_path(program) {
            Some(path) => return OutputType::Success(Some(format!("{} is {}\n", program, path))),
            None => return OutputType::Error(Some( format!("{}: not found\n", program))), 
        }
    }
}
fn run_pwd(redirection: &Option<Redirection>)->OutputType {
    // let pwd = get_pwd();
    // match redirection {
    //     Some(file) => redirect(file, &pwd),
    //     None => {
    //         println!("{}", pwd)
    //     }
    // }
    get_pwd()
}
fn get_pwd() -> OutputType {
    match std::env::current_dir() {
        Ok(path) => OutputType::Success(Some(format!("{}\n", path.display()))),
        Err(e) => OutputType::Error(Some(format!("Error getting current directory: {}\n", e))),
    }
}
fn run_cd(args: &Vec<String>, redirection: &Option<Redirection>)->OutputType {
    // let output_option = get_cd_output(args);
    // match output_option {
    //     Some(output) => match redirection {
    //         Some(file) => redirect(file, &output),
    //         None => {
    //             println!("{}", output)
    //         }
    //     },
    //     None => {}
    // }
    get_cd_output(args)
}
fn get_cd_output(args: &Vec<String>) -> OutputType {
    let target_dir = if args.is_empty() || args[0] == "~" {
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };
    if let Err(_) = std::env::set_current_dir(&target_dir) {
        return OutputType::Error(Some(format!(
            "cd: {}: {}\n",
            target_dir, "No such file or directory"
        )));
    }
    OutputType::Success(None)
}

fn run_external_programs(program: &str, args: &Vec<String>, redirection: &Option<Redirection>)->OutputType {
    // let output = get_external_program_output(program, args);
    // match redirection {
    //     Some(file) => redirect(file, &output),
    //     None => {
    //         print!("{}", output);
    //         if !output.ends_with('\n') && needs_newline(&output) {
    //             println!();
    //         }
    //     }
    // }
    get_external_program_output(program, args)

}
fn get_external_program_output(program: &str, args: &Vec<String>) -> OutputType {
    match search_in_path(program) {
        Some(path) => {
            let output = process::Command::new(path)
                .arg0(program)
                .args(args)
                .output()
                .expect("failed to execute process");
            // Convert stdout bytes to String
            let output_as_str = String::from_utf8_lossy(&output.stdout).to_string();
            let err: String = String::from_utf8_lossy(&output.stderr).to_string();

            if !err.is_empty() {
                return OutputType::Error(Some(format!("{}", err)))
            }

            OutputType::Success(Some(output_as_str))
        }
        None => OutputType::Error(Some(format!("{}: command not found\n", program)))
       
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
// fn redirect(redirection: &Redirection, output: &String) {
//     match redirection {
//         Redirection::OutputTruncate(file) => output_truncate_redirection(&file, output),
//         _ => panic!("to do"),
//     }
// }
// fn output_truncate_redirection(file: &String, output: &String) {
//     let path = Path::new(file);

//     // Ensure parent directories exist
//     if let Some(parent) = path.parent() {
//         if !parent.exists() {
//             if let Err(e) = fs::create_dir_all(parent) {
//                 eprint!("echo: cannot create directory {}: {}", parent.display(), e);
//                 return;
//             }
//         }
//     }

//     // This will create a new file or truncate the existing one
//     match File::create(path) {
//         Ok(mut f) => {
//             if let Err(e) = write!(f, "{}", output) {
//                 eprint!("echo: cannot write to {}: {}", file, e);
//             }
//         }
//         Err(e) => eprint!("echo: cannot create {}: {}", file, e),
//     }
// }

fn needs_newline(output: &str) -> bool {
    output
        .chars()
        .any(|c| !c.is_control() && !c.is_whitespace())
}
