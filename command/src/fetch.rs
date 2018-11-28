use crate::{Command, CommandBoxClone, WorkOption, WorkResult};
use color_printer::{Color, ColorPrinter, ColorSpec};
use command_derive::CommandBoxClone;
use gitlib::{GitRepo, Status};
use std::{io::Write, path::PathBuf};

#[derive(Clone, CommandBoxClone, Default)]
pub struct FetchCommand;

impl FetchCommand {
    pub fn new() -> Self {
        Self {}
    }
}

struct FetchCommandResult {}

impl Command for FetchCommand {
    fn process(&self, repo: GitRepo) -> WorkOption {
        Some(Box::new(FetchCommandResult {}))
    }
}

impl WorkResult for FetchCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {}
}
