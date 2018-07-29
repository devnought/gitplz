use crate::StatusEntry;
use git2;
use std::iter::FilterMap;

pub type StatusIter<'a> =
    FilterMap<git2::StatusIter<'a>, fn(git2::StatusEntry<'a>) -> Option<StatusEntry>>;

pub struct Statuses<'a> {
    statuses: git2::Statuses<'a>,
}

impl From<git2::Statuses<'a>> for Statuses<'a> {
    fn from(statuses: git2::Statuses<'a>) -> Self {
        Self { statuses }
    }
}

impl Statuses<'a> {
    pub fn iter(&self) -> StatusIter<'_> {
        let iter: StatusIter<'_> = self.statuses.iter().filter_map(|x| {
            let path = x.path()?.into();
            let status = x.status();

            Some(StatusEntry::new(path, status))
        });

        iter
    }

    pub fn is_empty(&self) -> bool {
        self.statuses.is_empty()
    }
}
