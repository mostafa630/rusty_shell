#[derive(Debug)]
pub enum Token {
    Word(String),
    Pipe,           // |
    Ampersand,      // &
    RedirectIn,     // <
    RedirectOut,    // >
    RedirectAppend, // >>
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_single_quotes = false;
        let mut in_double_quotes = false;

        let mut chars = input.chars().peekable();
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
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }
                    continue;
                }
                '1' => {
                    if let Some(&'>') = chars.peek() {
                        chars.next();
                        tokens.push(Token::RedirectOut);
                    } else {
                        current.push(c)
                    }
                    continue;
                }
                '>' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }
                    if let Some(&'>') = chars.peek() {
                        chars.next(); // that mean  >> exist not only >
                        tokens.push(Token::RedirectAppend);
                        current.clear();
                    } else {
                        tokens.push(Token::RedirectOut); // >  only exist
                        current.clear();
                    }
                    continue;
                }
                '<' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }
                    tokens.push(Token::RedirectIn);
                    continue;
                }
                '|' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }
                    tokens.push(Token::Pipe);
                    continue;
                }
                '&' if !in_single_quotes && !in_double_quotes => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }
                    tokens.push(Token::Ampersand);
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
            tokens.push(Token::Word(current.clone()));
        }
        tokens
    }
}
