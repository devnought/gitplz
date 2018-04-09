use std::path::{Path, PathBuf};
use super::{StatusEntryIter, git2};

#[derive(Debug)]
pub struct StatusEntry {
    path: PathBuf,
    status: git2::Status,
}

impl StatusEntry {
    pub(crate) fn new(path: PathBuf, status: git2::Status) -> Self {
        StatusEntry { path, status }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn iter(&self) -> StatusEntryIter {
        StatusEntryIter::new(&self.path, &self.status)
    }
}
