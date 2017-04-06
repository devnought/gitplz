extern crate git2;
extern crate term_painter;

use std::path::Path;
use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta};
use term_painter::ToStyle;

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

    pub fn path(&self) -> Option<&str> {
        self.entry.path()
    }

    pub fn status(&self) -> FileStatus {
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

pub struct GitIter<'a> {
    statuses: Option<git2::StatusIter<'a>>,
}

impl<'a> GitIter<'a> {
    fn new(statuses: &'a git2::Statuses) -> Self {
        GitIter { statuses: Some(statuses.iter()) }
    }

    fn empty() -> Self {
        GitIter { statuses: None }
    }
}

impl<'a> Iterator for GitIter<'a> {
    type Item = GitEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.statuses.as_mut() {
            Some(statuses) => statuses.next().map(GitEntry::new),
            None => None,
        }
    }
}

pub struct GitStatuses<'a> {
    statuses: Option<git2::Statuses<'a>>,
}

impl<'a> GitStatuses<'a> {
    fn new(statuses: git2::Statuses<'a>) -> Self {
        GitStatuses { statuses: Some(statuses) }
    }

    fn empty() -> Self {
        GitStatuses { statuses: None }
    }

    pub fn len(&self) -> usize {
        match self.statuses.as_ref() {
            Some(statuses) => statuses.len(),
            None => 0
        }
    }

    pub fn iter(&self) -> GitIter {
        match self.statuses.as_ref() {
            Some(statuses) => GitIter::new(statuses),
            None => GitIter::empty(),
        }
    }
}

pub struct GitRepo {
    repo: Option<git2::Repository>,
}

impl GitRepo {
    pub fn new(path: &Path) -> Result<Self, GitError> {
        let repo = git2::Repository::open(path).map_err(|_| GitError::OpenRepo)?;

        if repo.is_bare() {
            return Ok(GitRepo { repo: None });
        }

        Ok(GitRepo { repo: Some(repo) })
    }

    pub fn statuses(&self) -> Result<GitStatuses, GitError> {
        let repo = match self.repo.as_ref() {
            Some(r) => r,
            None => return Ok(GitStatuses::empty()),
        };

        let mut opts = git2::StatusOptions::new();

        opts.include_ignored(false)
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_unreadable_as_untracked(true)
            .disable_pathspec_match(true)
            .exclude_submodules(true);

        let statuses = repo.statuses(Some(&mut opts))
            .map_err(|_| GitError::Status)?;

        Ok(GitStatuses::new(statuses))
    }
}





pub fn changes(path: &Path) -> Result<(), GitError> {
    let repo = GitRepo::new(path)?;
    let statuses = repo.statuses()?;

    if statuses.len() == 0 {
        return Ok(());
    }

    println!("{}", path.to_str().unwrap());

    for entry in statuses.iter() {
        let (pre, colour) = match entry.status() {
            FileStatus::Deleted => ("    Deleted", BrightRed),
            FileStatus::Modified => ("   Modified", BrightCyan),
            FileStatus::New => ("        New", BrightGreen),
            FileStatus::Renamed => ("    Renamed", BrightCyan),
            FileStatus::Typechanged => ("Typechanged", BrightCyan),
            FileStatus::Unknown => ("    Unknown", BrightMagenta),
        };

        println!("  {} {}", colour.paint(pre), entry.path().unwrap());
    }

    Ok(())
}