use git2;

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
    path: String,
    status: FileStatus,
}

impl GitStatusEntry {
    pub fn new(entry: git2::StatusEntry) -> Self {
        let status = match entry.status() {
            git2::STATUS_CONFLICTED => FileStatus::Conflicted,
            git2::STATUS_CURRENT => FileStatus::Current,
            git2::STATUS_IGNORED => FileStatus::Ignored,

            git2::STATUS_INDEX_NEW => FileStatus::StagedNew,
            git2::STATUS_INDEX_MODIFIED => FileStatus::StagedModified,
            git2::STATUS_INDEX_DELETED => FileStatus::StagedDeleted,
            git2::STATUS_INDEX_RENAMED => FileStatus::StagedRenamed,
            git2::STATUS_INDEX_TYPECHANGE => FileStatus::StagedTypechange,

            git2::STATUS_WT_DELETED => FileStatus::Deleted,
            git2::STATUS_WT_MODIFIED => FileStatus::Modified,
            git2::STATUS_WT_NEW => FileStatus::New,
            git2::STATUS_WT_RENAMED => FileStatus::Renamed,
            git2::STATUS_WT_TYPECHANGE => FileStatus::Typechange,

            _ => FileStatus::Unknown,
        };

        Self {
            path: entry.path().unwrap().to_string(),
            status: status,
        }
    }

    pub fn path(&self) -> Option<&str> {
        Some(&self.path)
    }

    pub fn status(&self) -> &FileStatus {
        &self.status
    }
}
