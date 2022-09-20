use crate::args::Class;
use hidg::{Button, Key};
use rustyline::{
    completion::{Candidate, Completer},
    highlight::Highlighter,
    hint::{Hint, Hinter},
    validate::Validator,
    Context, Helper, Result,
};

pub struct Cli {
    class: Class,
    keys: Vec<Entry>,
}

impl Helper for Cli {}

impl Cli {
    pub fn new(class: Class) -> Self {
        let keys = match class {
            Class::Keyboard => hidg::Key::VARIANTS
                .iter()
                .copied()
                .map(Entry::Key)
                .collect(),
            Class::Mouse => hidg::Button::VARIANTS
                .iter()
                .copied()
                .map(Entry::Btn)
                .collect(),
        };

        Self { class, keys }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Command {
    State,
    Press,
    Release,
    Move,
    Wheel,
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        use Command::*;
        match self {
            State => "state",
            Press => "press",
            Release => "release",
            Move => "move",
            Wheel => "wheel",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Entry {
    Cmd(Command),
    Key(Key),
    Btn(Button),
}

impl AsRef<str> for Entry {
    fn as_ref(&self) -> &str {
        use Entry::*;
        match self {
            Cmd(cmd) => cmd.as_ref(),
            Key(key) => key.as_ref(),
            Btn(btn) => btn.as_ref(),
        }
    }
}

impl Hinter for Cli {
    type Hint = Entry;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for Cli {}

impl Validator for Cli {}

impl Hint for Entry {
    fn display(&self) -> &str {
        self.as_ref()
    }
    fn completion(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl Candidate for Entry {
    fn display(&self) -> &str {
        self.as_ref()
    }
    fn replacement(&self) -> &str {
        self.as_ref()
    }
}

impl Completer for Cli {
    type Candidate = Entry;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        let line = line.split_at(pos).0;
        if let Some((cmd, args)) = line.split_once(' ') {
            let pos = cmd.len()
                + 1
                + if let Some(pos) = args.rfind(' ') {
                    pos
                } else {
                    0
                };
            match cmd {
                "press" | "release" => Ok((pos, self.keys.clone())),
                _ => Ok((0, vec![])),
            }
        } else {
            Ok((
                0,
                match self.class {
                    Class::Keyboard => vec![
                        Entry::Cmd(Command::State),
                        Entry::Cmd(Command::Press),
                        Entry::Cmd(Command::Release),
                    ],
                    Class::Mouse => vec![
                        Entry::Cmd(Command::State),
                        Entry::Cmd(Command::Press),
                        Entry::Cmd(Command::Release),
                        Entry::Cmd(Command::Move),
                        Entry::Cmd(Command::Wheel),
                    ],
                },
            ))
        }
    }
}
