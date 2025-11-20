use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::History;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct AutoCompleter {
    commands: Vec<String>,
}

impl AutoCompleter {
    pub fn new() -> Self {
        Self {
            commands: vec!["echo".into(), "exit".into()],
        }
    }
}
impl Completer for AutoCompleter {
    type Candidate = Pair;
    
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = 0 ; 
        let prefix = &line[..pos];
        let matched: Vec<Pair> = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: format!("{} ", cmd), // Note the space at the end
            })
            .collect();

        Ok((start, matched))
    }
    
    fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str, cl: &mut rustyline::Changeset) {
        let end = line.pos();
        line.replace(start..end, elected, cl);
    }
    
}

impl Hinter for AutoCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> { None }
}

impl Highlighter for AutoCompleter {}
impl Validator for AutoCompleter {}
impl Helper for AutoCompleter {}
impl History for AutoCompleter{
    fn get(&self, index: usize, dir: rustyline::history::SearchDirection) -> rustyline::Result<Option<rustyline::history::SearchResult>> {
        todo!()
    }

    fn add(&mut self, line: &str) -> rustyline::Result<bool> {
        todo!()
    }

    fn add_owned(&mut self, line: String) -> rustyline::Result<bool> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn set_max_len(&mut self, len: usize) -> rustyline::Result<()> {
        todo!()
    }

    fn ignore_dups(&mut self, yes: bool) -> rustyline::Result<()> {
        todo!()
    }

    fn ignore_space(&mut self, yes: bool) {
        todo!()
    }

    fn save(&mut self, path: &std::path::Path) -> rustyline::Result<()> {
        todo!()
    }

    fn append(&mut self, path: &std::path::Path) -> rustyline::Result<()> {
        todo!()
    }

    fn load(&mut self, path: &std::path::Path) -> rustyline::Result<()> {
        todo!()
    }

    fn clear(&mut self) -> rustyline::Result<()> {
        todo!()
    }

    fn search(
        &self,
        term: &str,
        start: usize,
        dir: rustyline::history::SearchDirection,
    ) -> rustyline::Result<Option<rustyline::history::SearchResult>> {
        todo!()
    }

    fn starts_with(
        &self,
        term: &str,
        start: usize,
        dir: rustyline::history::SearchDirection,
    ) -> rustyline::Result<Option<rustyline::history::SearchResult>> {
        todo!()
    }
}
