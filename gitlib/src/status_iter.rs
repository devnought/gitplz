use super::{git2, GitStatusEntry};

pub struct GitStatusIter<'a> {
    statuses: Option<git2::StatusIter<'a>>,
}

impl<'a> GitStatusIter<'a> {
    pub fn new(statuses: &'a git2::Statuses) -> Self {
        GitStatusIter { statuses: Some(statuses.iter()) }
    }
}

impl<'a> Iterator for GitStatusIter<'a> {
    type Item = GitStatusEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.statuses.as_mut() {
            Some(statuses) => statuses.next().map(GitStatusEntry::new),
            None => None,
        }
    }
}
