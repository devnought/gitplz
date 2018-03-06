use git2;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum FileStatus {
    Conflicted,
    Current,
    Deleted,
    Ignored,
    Modified,
    New,
    Renamed,
    StagedDeleted,
    StagedModified,
    StagedNew,
    StagedRenamed,
    StagedTypechange,
    Typechange,
    Unknown,
}

pub struct GitStatusEntry {
    path: PathBuf,
    status: FileStatus,
}

impl GitStatusEntry {
    pub fn new(entry: &git2::StatusEntry) -> Self {
        let status = match entry.status() {
            git2::Status::CONFLICTED => FileStatus::Conflicted,
            git2::Status::CURRENT => FileStatus::Current,
            git2::Status::IGNORED => FileStatus::Ignored,

            git2::Status::INDEX_NEW => FileStatus::StagedNew,
            git2::Status::INDEX_MODIFIED => FileStatus::StagedModified,
            git2::Status::INDEX_DELETED => FileStatus::StagedDeleted,
            git2::Status::INDEX_RENAMED => FileStatus::StagedRenamed,
            git2::Status::INDEX_TYPECHANGE => FileStatus::StagedTypechange,

            git2::Status::WT_DELETED => FileStatus::Deleted,
            git2::Status::WT_MODIFIED => FileStatus::Modified,
            git2::Status::WT_NEW => FileStatus::New,
            git2::Status::WT_RENAMED => FileStatus::Renamed,
            git2::Status::WT_TYPECHANGE => FileStatus::Typechange,

            _ => FileStatus::Unknown,
        };

        let path = match entry.path() {
            Some(p) => PathBuf::from(p),
            None => PathBuf::new(),
        };

        Self {
            path: path,
            status: status,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn status(&self) -> &FileStatus {
        &self.status
    }
}
