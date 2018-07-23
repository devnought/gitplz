use git2;
use std::path::Path;

const STATUS_COLLECTION: [git2::Status; 12] = [
    git2::Status::INDEX_NEW,
    git2::Status::INDEX_MODIFIED,
    git2::Status::INDEX_DELETED,
    git2::Status::INDEX_RENAMED,
    git2::Status::INDEX_TYPECHANGE,
    git2::Status::WT_NEW,
    git2::Status::WT_MODIFIED,
    git2::Status::WT_DELETED,
    git2::Status::WT_TYPECHANGE,
    git2::Status::WT_RENAMED,
    git2::Status::IGNORED,
    git2::Status::CONFLICTED,
];

#[derive(Debug, Clone)]
pub enum Status {
    Conflicted,
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

impl Status {
    fn new(status: git2::Status) -> Self {
        match status {
            git2::Status::INDEX_NEW => Status::StagedNew,
            git2::Status::INDEX_MODIFIED => Status::StagedModified,
            git2::Status::INDEX_DELETED => Status::StagedDeleted,
            git2::Status::INDEX_RENAMED => Status::StagedRenamed,
            git2::Status::INDEX_TYPECHANGE => Status::StagedTypechange,
            git2::Status::WT_NEW => Status::New,
            git2::Status::WT_MODIFIED => Status::Modified,
            git2::Status::WT_DELETED => Status::Deleted,
            git2::Status::WT_TYPECHANGE => Status::Typechange,
            git2::Status::WT_RENAMED => Status::Renamed,
            git2::Status::IGNORED => Status::Ignored,
            git2::Status::CONFLICTED => Status::Conflicted,
            _ => Status::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct StatusEntryIter<'a> {
    status: git2::Status,
    path: &'a Path,
    index: i8,
    multiple_statuses: bool,
}

impl StatusEntryIter<'a> {
    pub(crate) fn new(path: &'a Path, status: git2::Status) -> Self {
        Self {
            index: -1,
            status,
            path,
            multiple_statuses: status.bits().count_ones() > 1,
        }
    }
}

impl Iterator for StatusEntryIter<'a> {
    type Item = (&'a Path, Status);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        // Optimization for single statuses
        if !self.multiple_statuses {
            if self.index == 0 {
                return Some((self.path, Status::new(self.status)));
            } else {
                return None;
            }
        }

        // Only make it here if there are multiple statuses
        let status = loop {
            if self.index as usize >= STATUS_COLLECTION.len() {
                return None;
            }

            let check = STATUS_COLLECTION[self.index as usize];

            if self.status.intersects(check) {
                break Status::new(check);
            }

            self.index += 1;
        };

        Some((self.path, status))
    }
}
