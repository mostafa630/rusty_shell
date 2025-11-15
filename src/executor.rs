use crate::{command::{OutputType, RedirectCode, Redirection}, parser::ParsedLine};
pub struct Executor;

impl Executor {
    pub fn execute(parsed_line: ParsedLine) {
        for cmd in parsed_line.pipeline.commands {
            let output_type = cmd.execute();
            Self::process_output(output_type, cmd.redirection);
        }
    }
    fn process_output(output_type: OutputType, redirection_option: Option<Redirection>) {
    match redirection_option {
        Some(redirection) => {
            match (output_type, redirection) {
                // Success output + redirect to code 1
                (OutputType::Success(Some(content)), Redirection::OutputTruncate(RedirectCode::One(file))) => {
                    std::fs::write(file, content).expect("Failed to write to file");
                },
                // Error output + redirect to code 2
                (OutputType::Error(Some(content)), Redirection::OutputTruncate(RedirectCode::Two(file))) => {
                    std::fs::write(file, content).expect("Failed to write to file");
                },
                // For mismatched redirection or None inside Success/Error, just print
                (output, _) => match output {
                    OutputType::Success(Some(content)) | OutputType::Error(Some(content)) => {
                        print!("{}", content);
                    },
                    OutputType::Success(None) | OutputType::Error(None) => {}
                },
            }
        },
        None => {
            // No redirection, just print
            match output_type {
                OutputType::Success(Some(content)) | OutputType::Error(Some(content)) => print!("{}", content),
                OutputType::Success(None) | OutputType::Error(None) => {}
            }
        }
    }
}

    
}
