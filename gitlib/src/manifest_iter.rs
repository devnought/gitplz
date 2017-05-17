use super::GitRepo;
use manifest::ManifestData;

use std::collections::btree_set::Iter;
use std::path::{Path, PathBuf};

pub struct ManifestIterator<'a> {
    iter: Iter<'a, PathBuf>,
    root: &'a Path
}

impl<'a> ManifestIterator<'a> {
    pub fn new(data: &'a ManifestData) -> Self {
        let iter = data.repos();
        let root = data.root();
        ManifestIterator { iter: iter.into_iter(), root: root }
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