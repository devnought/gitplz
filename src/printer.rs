use std::{io::Write, path::{Path, PathBuf}};

use gitlib::{self, Status};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use printopts::PrintOptions;
use worktype::{BranchOption, WorkResult};

pub struct Printer<'a> {
    print_options: &'a PrintOptions,
}

impl<'a> Printer<'a> {
    pub fn new(print_options: &'a PrintOptions) -> Self {
        Self { print_options }
    }

    pub fn handle(&self, message: &WorkResult) {
        match *message {
            WorkResult::Branch {
                ref path,
                ref branch,
                ref option,
                ref error,
            } => self.branch(path, branch, option, error),
            WorkResult::Checkout {
                ref path,
                ref branch,
            } => self.checkout(path, branch),
            WorkResult::Reset { ref path, ref head } => self.reset(path, head),
            WorkResult::Status {
                ref path,
                ref statuses,
            } => self.status(path, statuses),
        }
    }

    pub fn branch(
        &self,
        path: &Path,
        branch: &str,
        option: &BranchOption,
        error: &Option<gitlib::Error>,
    ) {
        match *option {
            BranchOption::Delete => self.branch_delete(path, branch, error),
            BranchOption::Find => self.branch_find(path, branch),
        }
    }

    fn branch_delete(&self, path: &Path, branch: &str, error: &Option<gitlib::Error>) {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut handle = stdout.lock();

        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Red));

        self.color_context(&cs, &mut handle, |h| {
            write!(h, " {}", branch).expect("write fail");

            if error.is_some() {
                write!(h, " - ERROR").expect("write fail");
            }
        });

        writeln!(handle, " - {}", path.display()).expect("write fail");
    }

    fn branch_find(&self, path: &Path, branch: &str) {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut handle = stdout.lock();

        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Green));

        self.color_context(&cs, &mut handle, |h| {
            write!(h, " {}", branch).expect("write fail")
        });

        writeln!(handle, " - {}", path.display()).expect("write fail");
    }

    pub fn checkout(&self, path: &Path, branch: &str) {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut handle = stdout.lock();

        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Yellow));

        self.color_context(&cs, &mut handle, |h| {
            write!(h, " {}", branch).expect("write fail")
        });

        writeln!(handle, " - {}", path.display()).expect("write fail");
    }

    pub fn reset(&self, path: &Path, head: &str) {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut handle = stdout.lock();

        let mut cs = ColorSpec::new();
        cs.set_intense(true);
        cs.set_fg(Some(Color::Yellow));

        self.color_context(&cs, &mut handle, |h| {
            write!(h, " {}", head).expect("write fail")
        });

        writeln!(handle, " - {}", path.display()).expect("write fail");
    }

    pub fn status(&self, path: &Path, statuses: &[(PathBuf, Status)]) {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut handle = stdout.lock();

        writeln!(handle, "{}", path.display()).expect("write fail");

        let mut cs = ColorSpec::new();
        cs.set_intense(true);

        for &(ref path, ref status) in statuses {
            let (status_str, color) = match *status {
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

            self.color_context(&cs, &mut handle, |h| {
                write!(h, " {}", status_str).expect("write fail");
            });

            writeln!(handle, " {}", path.display()).expect("write fail");
        }
    }

    fn color_context<C, F>(&self, color_spec: &ColorSpec, handle: &mut C, func: F)
    where
        C: Write + WriteColor,
        F: Fn(&mut Write) -> (),
    {
        if !self.print_options.is_terminal() {
            func(handle);
            return;
        }

        handle.set_color(color_spec).expect("color set fail");
        func(handle);
        handle.reset().expect("Color reset fail");
    }
}
