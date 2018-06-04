use color_printer::{Color, ColorPrinter, ColorSpec};
use command::Command;
use gitlib::{self, GitRepo};
use std::{io::Write, path::PathBuf};
use worktype::{WorkResult, WorkType};

#[derive(Clone)]
pub struct BranchDeleteCommand {
    branch: String,
}

impl BranchDeleteCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }

    pub fn box_new(branch: String) -> Box<Self> {
        Box::new(Self::new(branch))
    }
}

pub struct BranchDeleteCommandResult {
    path: PathBuf,
    branch: String,
    error: Option<gitlib::Error>,
}

impl Command for BranchDeleteCommand {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType {
        let result = match repo.delete_local_branch(&self.branch) {
            Ok(()) => BranchDeleteCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
                error: None,
            },
            Err(gitlib::Error::NotFound) => return WorkType::empty(index),
            Err(e) => BranchDeleteCommandResult {
                path: repo.path().into(),
                branch: self.branch.clone(),
                error: Some(e),
            },
        };

        WorkType::result(index, Box::new(result))
    }

    fn box_clone(&self) -> Box<Command> {
        Box::new(self.clone())
    }
}

impl WorkResult for BranchDeleteCommandResult {
    fn print(&self, printer: &mut ColorPrinter) {
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
