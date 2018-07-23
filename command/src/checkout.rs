use crate::{WorkResult, WorkType, Command};
use color_printer::{Color, ColorPrinter, ColorSpec};
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};
use command_derive::CommandBoxClone;
use crate::command::CommandBoxClone;

#[derive(Clone, CommandBoxClone)]
pub struct CheckoutCommand {
    branch: String,
}

impl CheckoutCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }
}

struct CheckoutCommandResult {
    branch: String,
    path: PathBuf,
}

impl Command for CheckoutCommand {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType {
        if let Ok(true) = repo.checkout(&self.branch) {
            let result = CheckoutCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
            };
            WorkType::result(index, Box::new(result))
        } else {
            WorkType::empty(index)
        }
    }
}

impl WorkResult for CheckoutCommandResult {
    fn print(&self, printer: &mut ColorPrinter) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Yellow));

        printer.color_context(&cs, |h| write!(h, " {}", self.branch).expect("write fail"));

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
