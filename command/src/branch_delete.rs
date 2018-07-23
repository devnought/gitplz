use color_printer::{Color, ColorPrinter, ColorSpec};
use command_derive::CommandBoxClone;
use crate::{Command, CommandBoxClone, WorkOption, WorkResult};
use gitlib::{self, GitRepo};
use std::{io::Write, path::PathBuf};

#[derive(Clone, CommandBoxClone)]
pub struct BranchDeleteCommand {
    branch: String,
}

impl BranchDeleteCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }
}

struct BranchDeleteCommandResult {
    path: PathBuf,
    branch: String,
    error: Option<gitlib::Error>,
}

impl Command for BranchDeleteCommand {
    fn process(&self, repo: GitRepo) -> WorkOption {
        let result = match repo.delete_local_branch(&self.branch) {
            Ok(()) => BranchDeleteCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
                error: None,
            },
            Err(gitlib::Error::NotFound) => return None,
            Err(e) => BranchDeleteCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
                error: Some(e),
            },
        };

        Some(Box::new(result))
    }
}

impl WorkResult for BranchDeleteCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Red));

        printer.color_context(&cs, |h| {
            write!(h, " {}", self.branch).expect("write fail");

            if self.error.is_some() {
                write!(h, " - ERROR").expect("write fail");
            }
        });

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
