use crate::{WorkResult, Command};
use color_printer::{Color, ColorPrinter, ColorSpec};
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};
use command_derive::CommandBoxClone;
use crate::command::{CommandBoxClone, WorkOption};

#[derive(Clone, CommandBoxClone)]
pub struct BranchFindCommand {
    branch: String,
}

impl BranchFindCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }
}

struct BranchFindCommandResult {
    branch: String,
    path: PathBuf,
}

impl Command for BranchFindCommand {
    fn process(&self, repo: GitRepo) -> WorkOption {
        if let Ok(()) = repo.has_local_branch(&self.branch) {
            let result = Box::new(BranchFindCommandResult {
                branch: self.branch.clone(),
                path: repo.path().into(),
            });

            Some(result)
        } else {
            None
        }
    }
}

impl WorkResult for BranchFindCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Green));

        printer.color_context(&cs, |h| write!(h, " {}", self.branch).expect("write fail"));

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
