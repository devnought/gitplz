use super::git2;

#[derive(Debug)]
pub enum FileStatus {
    Deleted,
    Modified,
    New,
    Renamed,
    Typechanged,
    Unknown,
}

pub struct GitStatusEntry<'a> {
    entry: git2::StatusEntry<'a>,
}

impl<'a> GitStatusEntry<'a> {
    pub fn new(entry: git2::StatusEntry<'a>) -> Self {
        GitStatusEntry { entry: entry }
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