use gitlib::GitRepo;
use manifest::ManifestData;

use std::collections::btree_set::Iter;
use std::path::Path;

pub struct ManifestIterator<'a> {
    iter: Iter<'a, String>,
    root: &'a Path
}

impl<'a> ManifestIterator<'a> {
    pub fn new(data: &'a ManifestData) -> Self {
        let di = &data.repos;
        let dr = &data.root;
        ManifestIterator { iter: di.into_iter(), root: dr }
    }
}

impl<'a> Iterator for ManifestIterator<'a> {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next().map(|x| self.root.join(x)) {
            Some(p) => Some(GitRepo::new(p).unwrap()),
            None => None,
        }
    }
}