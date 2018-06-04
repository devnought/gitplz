use gitlib::GitRepo;
use std::path::PathBuf;

use repo_iter_state::RepoIterState;

pub struct RepoIter {
    state: RepoIterState,
}

impl RepoIter {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            state: RepoIterState::new(path),
        }
    }
}

impl Iterator for RepoIter {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iter_result = {
                let mut iter = match self.state.get_iter() {
                    None => break None,
                    Some(iter) => iter,
                };

                iter.next()
            };

            let entry = match iter_result {
                None | Some(Err(_)) => {
                    self.state.end_iter();
                    continue;
                }
                Some(Ok(entry)) => entry,
            };

            if let Ok(repo) = GitRepo::open(entry.path()) {
                break Some(repo);
            }

            self.state.add_pending(entry.path());
        }
    }
}
