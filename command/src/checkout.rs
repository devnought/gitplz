use color_printer::{Color, ColorPrinter, ColorSpec};
use command_derive::CommandBoxClone;
use crate::command::{CommandBoxClone, WorkOption};
use crate::{Command, WorkResult};
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};

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
    fn process(&self, repo: GitRepo) -> WorkOption {
        if let Ok(true) = repo.checkout(&self.branch) {
            let result = CheckoutCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
            };

            Some(Box::new(result))
        } else {
            None
        }
    }
}

impl WorkResult for CheckoutCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Yellow));

        printer.color_context(&cs, |h| write!(h, " {}", self.branch).expect("write fail"));

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
