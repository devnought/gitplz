use crate::{Command, CommandBoxClone, WorkOption, WorkResult};
use color_printer::{Color, ColorPrinter, ColorSpec};
use command_derive::CommandBoxClone;
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};

#[derive(Clone, CommandBoxClone, Default)]
pub struct FetchCommand;

impl FetchCommand {
    pub fn new() -> Self {
        Self {}
    }
}

struct FetchCommandResult {
    msg: String,
}

impl Command for FetchCommand {
    fn process(&self, repo: GitRepo) -> WorkOption {
        let res = match repo.fetch() {
            Ok(_) => Box::new(FetchCommandResult {
                msg: "My man!".into(),
            }),
            Err(_) => Box::new(FetchCommandResult {
                msg: "Not like this".into(),
            }),
        };

        Some(res)
    }
}

impl WorkResult for FetchCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {
        writeln!(printer, "{}", self.msg).expect("write fail");
    }
}
