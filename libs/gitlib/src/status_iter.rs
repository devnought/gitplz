use super::{GitStatusEntry, git2};

pub struct GitStatusIter<'a> {
    statuses: Option<git2::StatusIter<'a>>,
}

impl<'a> GitStatusIter<'a> {
    pub fn new(statuses: &'a git2::Statuses) -> Self {
        Self {
            statuses: Some(statuses.iter()),
        }
    }
}

impl<'a> Iterator for GitStatusIter<'a> {
    type Item = GitStatusEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.statuses.as_mut() {
            Some(statuses) => if let Some(s) = statuses.next() {
                Some(GitStatusEntry::new(&s))
            } else {
                None
            },
            None => None,
        }
    }
}
