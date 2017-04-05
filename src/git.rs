extern crate git2;
extern crate term_painter;

use std::path::Path;
use std::iter;
use std::marker::PhantomData;

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

pub struct GitIter<'a> {
    iter: Option<git2::StatusIter<'a>>,
}

impl<'a> GitIter<'a> {
    fn new(statuses: &'a git2::Statuses) -> Self {
        GitIter { iter: Some(statuses.iter()) }
    }

    fn empty() -> Self {
        GitIter { iter: None }
    }
}

impl<'a> Iterator for GitIter<'a> {
    type Item = GitEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.as_mut() {
            Some(iter) => iter.next().map(GitEntry::new),
            None => None,
        }
    }
}

/*pub fn changes_test<'a>(path: &Path) -> Result<GitIter<'a>, GitError> {
    let repo = git2::Repository::open(path).map_err(|_| GitError::OpenRepo)?;

    if repo.is_bare() {
        return Ok(GitIter::empty());
    }

    let mut opts = git2::StatusOptions::new();

    opts.include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_unreadable_as_untracked(true)
        .disable_pathspec_match(true)
        .exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts))
        .map_err(|_| GitError::Status)?;

    let iter = GitIter::new(&statuses);

    Ok(iter)
}*/

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

    let statuses = repo.statuses(Some(&mut opts))
        .map_err(|_| GitError::Status)?;

    let mut statuses_iter = GitIter::new(&statuses).peekable();

    if statuses_iter.peek().is_none() {
        return Ok(());
    }

    println!("{}", path.to_str().unwrap());

    for entry in statuses_iter {
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
