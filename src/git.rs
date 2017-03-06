extern crate git2;
extern crate colored;

use std::path::Path;
use std::iter::Filter;
use std::slice::Iter;
use colored::Colorize;

#[derive(Debug)]
pub enum GitError {
    OpenRepo,
    Status,
}

#[derive(Debug)]
pub enum FileStatus {
    Deleted,
    Modified,
    New,
    Renamed,
    Typechanged,
    Unknown,
}

pub struct GitEntry<'a> {
    entry: git2::StatusEntry<'a>,
}

impl<'a> GitEntry<'a> {
    fn new(entry: git2::StatusEntry<'a>) -> Self {
        GitEntry { entry: entry }
    }

    fn path(&self) -> Option<&str> {
        self.entry.path()
    }

    fn status(&self) -> FileStatus {
        match self.entry.status() {
            git2::STATUS_WT_DELETED => FileStatus::Deleted,
            git2::STATUS_WT_MODIFIED => FileStatus::Modified,
            git2::STATUS_WT_NEW => FileStatus::New,
            git2::STATUS_WT_RENAMED => FileStatus::Renamed,
            git2::STATUS_WT_TYPECHANGE => FileStatus::Typechanged,
            _ => FileStatus::Unknown,
        }
    }
}

// This can become its own iterator
pub fn changes(path: &Path) -> Result<(), GitError> {
    let repo = git2::Repository::open(path).map_err(|_| GitError::OpenRepo)?;

    if repo.is_bare() {
        return Ok(());
    }

    let mut opts = git2::StatusOptions::new();

    opts.include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_unreadable_as_untracked(true)
        .disable_pathspec_match(true)
        .exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts)).map_err(|_| GitError::Status)?;
    let mut statuses_iter = statuses.iter()
        .filter(|x| {
            if x.status() != git2::STATUS_WT_DELETED {
                return true;
            }

            // For some reason, some files with the deleted
            // status actually still exist, so ignore these.
            match x.path() {
                None => false,
                Some(p) => {
                    let mut del_path = path.to_path_buf();
                    del_path.push(p);
                    !del_path.exists()
                }
            }
        })
        .map(GitEntry::new)
        .peekable();

    if statuses_iter.peek().is_none() {
        return Ok(());
    }

    println!("{}", path.to_str().unwrap());

    for entry in statuses_iter {
        let pre = match entry.status() {
            FileStatus::Deleted => "    Deleted".red().bold(),
            FileStatus::Modified => "   Modified".cyan().bold(),
            FileStatus::New => "        New".green().bold(),
            FileStatus::Renamed => "    Renamed".cyan().bold(),
            FileStatus::Typechanged => "Typechanged".cyan().bold(),
            FileStatus::Unknown => "    Unknown".magenta().bold(),
        };

        println!("  {} {}", pre, entry.path().unwrap());
    }

    Ok(())
}