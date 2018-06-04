use super::{git2, StatusEntry};
use std::iter::FilterMap;

pub type StatusIter<'a> =
    FilterMap<git2::StatusIter<'a>, fn(git2::StatusEntry<'a>) -> Option<StatusEntry>>;

pub struct Statuses<'a> {
    statuses: git2::Statuses<'a>,
}

impl<'a> From<git2::Statuses<'a>> for Statuses<'a> {
    fn from(statuses: git2::Statuses<'a>) -> Self {
        Self { statuses }
    }
}

impl<'a> Statuses<'a> {
    pub fn iter(&self) -> StatusIter {
        let iter: StatusIter = self.statuses.iter().filter_map(status_entry_map);

        iter
    }

    pub fn is_empty(&self) -> bool {
        self.statuses.is_empty()
    }
}

fn status_entry_map(entry: git2::StatusEntry) -> Option<StatusEntry> {
    let path = entry.path()?.into();
    let status = entry.status();

    Some(StatusEntry::new(path, status))
}
