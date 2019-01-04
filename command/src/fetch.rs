use crate::{Command, CommandBoxClone, WorkOption, WorkResult};
use color_printer::ColorPrinter;
use command_derive::CommandBoxClone;
use gitlib::GitRepo;
use std::io::Write;

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
            Err(e) => Box::new(FetchCommandResult {
                msg: format!("{:?}", e),
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
