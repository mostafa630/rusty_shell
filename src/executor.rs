use std::fs::File;

use crate::{
    command::{CommandOutput, RedirectCode, Redirection},
    parser::ParsedLine,
};
pub struct Executor;

impl Executor {
    pub fn execute(parsed_line: ParsedLine) {
        for cmd in parsed_line.pipeline.commands {
            let output_type = cmd.execute();
            Self::process_output(output_type, cmd.redirection);
        }
    }
    fn process_output(output: CommandOutput, redirection: Option<Redirection>) {
        match redirection {
            Some(Redirection::OutputTruncate(RedirectCode::One(file))) => {
                File::create(&file).expect("can't create file");
                // redirect stdout only
                if let Some(content) = output.success {
                    std::fs::write(file, content).expect("Failed to write to file");
                }
                // print errors to console
                if let Some(err) = output.error {
                    eprint!("{}", err);
                }
            }
            Some(Redirection::OutputTruncate(RedirectCode::Two(file))) => {
                File::create(&file).expect("can't create file");
                // redirect stderr only
                if let Some(err) = output.error {
                    std::fs::write(file, err).expect("Failed to write error file");
                }
                // print successful output to console
                if let Some(content) = output.success {
                    print!("{}", content);
                }
            }
            Some(Redirection::OutputAppend(RedirectCode::One(file))) => {
                use std::io::Write;

                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file)
                    .expect("can't open file for append");

                if let Some(content) = output.success {
                    f.write_all(content.as_bytes())
                        .expect("failed to append to file");
                }
                if let Some(err) = output.error {
                    eprint!("{}", err);
                }
            }

            Some(Redirection::OutputAppend(RedirectCode::Two(file))) => {
                use std::io::Write;

                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file)
                    .expect("can't open file for append");

                if let Some(err) = output.error {
                    f.write_all(err.as_bytes())
                        .expect("failed to append error to file");
                }
                if let Some(content) = output.success {
                    print!("{}", content);
                }
            }
            None => {
                // no redirection: print both
                if let Some(content) = output.success {
                    print!("{}", content);
                }
                if let Some(err) = output.error {
                    eprint!("{}", err);
                }
            }
            _ => {
                panic!("to be handeled")
            }
        }
    }
}
