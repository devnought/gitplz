use super::{worktype::{WorkResult, WorkType},
            Command};
use color_printer::{Color, ColorPrinter, ColorSpec};
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};

#[derive(Clone)]
pub struct BranchFindCommand {
    branch: String,
}

impl BranchFindCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }

    pub fn box_new(branch: String) -> Box<Self> {
        Box::new(Self::new(branch))
    }
}

pub struct BranchFindCommandResult {
    branch: String,
    path: PathBuf,
}

impl Command for BranchFindCommand {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType {
        if let Ok(()) = repo.has_local_branch(&self.branch) {
            let result = Box::new(BranchFindCommandResult {
                branch: self.branch.clone(),
                path: repo.path().into(),
            });

            WorkType::result(index, result)
        } else {
            WorkType::empty(index)
        }
    }

    fn box_clone(&self) -> Box<Command> {
        Box::new(self.clone())
    }
}

impl WorkResult for BranchFindCommandResult {
    fn print(&self, printer: &mut ColorPrinter) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Green));

        printer.color_context(&cs, |h| write!(h, " {}", self.branch).expect("write fail"));

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
