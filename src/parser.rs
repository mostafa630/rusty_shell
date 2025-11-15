use crate::command::{Command, RedirectCode, Redirection};
use crate::tokenizer::Token;

#[derive(Debug)]

pub struct Pipeline {
    pub commands: Vec<Command>,
}
#[derive(Debug)]
pub enum ExecMode {
    Foreground,
    Background, // when you type & that mean you want this program run in Background
}

#[derive(Debug)]
pub struct ParsedLine {
    pub pipeline: Pipeline,
    pub mode: ExecMode,
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> ParsedLine {
        let mut commands = Vec::new();
        let mut args = Vec::new();
        let mut program = None;
        let mut redirection = None;
        let mut exec_mode = ExecMode::Foreground;

        let mut iter = tokens.into_iter().peekable();
        while let Some(token) = iter.next() {
            match token {
                Token::Word(word) => {
                    if program.is_none() {
                        program = Some(word)
                    } else {
                        args.push(word);
                    }
                }
                Token::RedirectIn => {
                    if let Some(Token::Word(file)) = iter.next() {
                        redirection = Some(Redirection::Input(file));
                    }
                }
                Token::RedirectOut => {
                    if let Some(Token::Word(file)) = iter.next() {
                        redirection = Some(Redirection::OutputTruncate(RedirectCode::One(file)));
                    }
                }
                Token::RedirectErrOut => {
                    if let Some(Token::Word(file)) = iter.next() {
                        redirection = Some(Redirection::OutputTruncate(RedirectCode::Two(file)));
                    }
                }
                Token::RedirectAppend => {
                    if let Some(Token::Word(file)) = iter.next() {
                        redirection = Some(Redirection::OutputAppend(file));
                    }
                }
                Token::Pipe => {
                    if let Some(_program) = program.take() {
                        // take : take the value and  make the program = None
                        commands.push(Command {
                            program: _program,
                            args: args,
                            redirection: redirection.take(),
                        });
                        args = Vec::new();
                    }
                }
                Token::Ampersand => {
                    exec_mode = ExecMode::Background;
                }
            }
        }
        // Push last command if exist
        if let Some(_program) = program.take() {
            commands.push(Command {
                program: _program,
                args,
                redirection: redirection.take(),
            });
        }
        ParsedLine {
            pipeline: Pipeline { commands },
            mode: exec_mode,
        }
    }
}
