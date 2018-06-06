use super::{worktype::{WorkResult, WorkType},
            Command};
use color_printer::{Color, ColorPrinter, ColorSpec};
use gitlib::{GitRepo, Status};
use std::{io::Write, path::PathBuf};

#[derive(Clone)]
pub struct StatusCommand {}

impl StatusCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn box_new() -> Box<Self> {
        Box::new(Self::new())
    }
}

struct StatusCommandResult {
    statuses: Vec<(PathBuf, Status)>,
    path: PathBuf,
}

impl Command for StatusCommand {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType {
        let statuses = match repo.statuses() {
            Err(_) => return WorkType::empty(index),
            Ok(ref s) if s.is_empty() => return WorkType::empty(index),
            Ok(s) => s,
        };

        let mut result = Vec::new();

        for status_entry in statuses.iter() {
            for (path, status) in status_entry.iter() {
                result.push((path.to_owned(), status));
            }
        }

        let result = Box::new(StatusCommandResult {
            path: repo.path().into(),
            statuses: result,
        });

        WorkType::result(index, result)
    }

    fn box_clone(&self) -> Box<Command> {
        Box::new(self.clone())
    }
}

impl WorkResult for StatusCommandResult {
    fn print(&self, printer: &mut ColorPrinter) {
        writeln!(printer, "{}", self.path.display()).expect("write fail");

        let mut cs = ColorSpec::new();
        cs.set_intense(true);

        for (path, status) in &self.statuses {
            let (status_str, color) = match status {
                Status::Conflicted => ("       Conflicted", Color::Magenta),
                Status::Deleted => ("          Deleted", Color::Red),
                Status::Ignored => ("          Ignored", Color::Magenta),
                Status::Modified => ("         Modified", Color::Cyan),
                Status::New => ("              New", Color::Green),
                Status::Renamed => ("          Renamed", Color::Green),
                Status::StagedDeleted => ("   Staged Deleted", Color::Magenta),
                Status::StagedModified => ("  Staged Modified", Color::Magenta),
                Status::StagedNew => ("       Staged New", Color::Magenta),
                Status::StagedRenamed => ("   Staged Renamed", Color::Magenta),
                Status::StagedTypechange => ("Staged Typechange", Color::Magenta),
                Status::Typechange => ("       Typechange", Color::Cyan),
                Status::Unknown => ("          Unknown", Color::Magenta),
            };

            cs.set_fg(Some(color));

            printer.color_context(&cs, |h| {
                write!(h, " {}", status_str).expect("write fail");
            });

            writeln!(printer, " {}", path.display()).expect("write fail");
        }
    }
}
