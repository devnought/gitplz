use color_printer::{Color, ColorPrinter, ColorSpec};
use command::Command;
use gitlib::GitRepo;
use std::{io::Write, path::PathBuf};
use worktype::{WorkResult, WorkType};

#[derive(Clone)]
pub struct CheckoutCommand {
    branch: String,
}

impl CheckoutCommand {
    pub fn new(branch: String) -> Self {
        Self { branch }
    }

    pub fn box_new(branch: String) -> Box<Self> {
        Box::new(Self::new(branch))
    }
}

pub struct CheckoutCommandResult {
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

    fn box_clone(&self) -> Box<Command> {
        Box::new(self.clone())
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
