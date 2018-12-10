use crate::{Command, CommandBoxClone, WorkOption, WorkResult};
use color_printer::{Color, ColorPrinter, ColorSpec};
use command_derive::CommandBoxClone;
use gitlib::{GitRepo, Status};
use std::{fs, io::Write, path::PathBuf};

#[derive(Clone, CommandBoxClone, Default)]
pub struct ResetCommand;

impl ResetCommand {
    pub fn new() -> Self {
        Self {}
    }
}

struct ResetCommandResult {
    path: PathBuf,
    head: String,
}

impl Command for ResetCommand {
    fn process(&self, repo: GitRepo) -> WorkOption {
        // If we can get the status of the repo, try that first
        // instead of blindly resetting when it's not required.
        let status_result = repo.statuses();

        let statuses = match status_result {
            Err(_) => None,
            Ok(s) => {
                if s.is_empty() {
                    return None;
                }

                Some(s)
            }
        };

        // Check for any 'new' files to delete
        if let Some(s) = statuses {
            let iter = s
                .iter()
                .filter(|x| {
                    for status in x.iter() {
                        if let (_, Status::New) = status {
                            return true;
                        }
                    }

                    false
                })
                .map(|x| repo.path().join(x.path()));

            for path in iter {
                fs::remove_file(path).expect("Could not remove file");
            }
        }

        // Proceed with normal reset
        let head = match repo.reset() {
            Err(_) => return None,
            Ok(h) => h,
        };

        let result = ResetCommandResult {
            path: repo.path().into(),
            head: head.name().into(),
        };

        Some(Box::new(result))
    }
}

impl WorkResult for ResetCommandResult {
    fn print(&self, printer: &mut ColorPrinter<'_>) {
        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Yellow));

        printer.color_context(&cs, |h| write!(h, " {}", self.head).expect("write fail"));

        writeln!(printer, " - {}", self.path.display()).expect("write fail");
    }
}
