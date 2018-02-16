use super::{GitStatusIter, git2};

pub struct GitStatuses<'a> {
    statuses: git2::Statuses<'a>,
}

impl<'a> GitStatuses<'a> {
    pub fn new(statuses: git2::Statuses<'a>) -> Self {
        Self { statuses: statuses }
    }

    pub fn len(&self) -> usize {
        self.statuses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.statuses.len() == 0
    }

    pub fn iter(&self) -> GitStatusIter {
        GitStatusIter::new(&self.statuses)
    }
}
