use crate::parser::ParsedLine;
pub struct Executor;

impl Executor {
    pub fn execute(parsed_line: ParsedLine) {
        for cmd in parsed_line.pipeline.commands {
            cmd.execute();
        }
    }
}
