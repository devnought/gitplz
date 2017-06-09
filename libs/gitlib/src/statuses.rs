use super::{git2, GitStatusIter};

pub struct GitStatuses<'a> {
    statuses: git2::Statuses<'a>,
}

impl<'a> GitStatuses<'a> {
    pub fn new(statuses: git2::Statuses<'a>) -> Self {
        GitStatuses { statuses: statuses }
    }

    pub fn len(&self) -> usize {
        self.statuses.len()
    }

    pub fn iter(&self) -> GitStatusIter {
        GitStatusIter::new(&self.statuses)
    }
}
