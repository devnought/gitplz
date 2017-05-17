#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate git2;

#[derive(Debug)]
pub enum GitError {
    Checkout(GitBranch),
    Manifest,
    OpenRepo,
    Reset,
    Status,
}

#[derive(Debug)]
pub enum GitBranch {
    Local,
    Remote,
}

impl From<git2::BranchType> for GitBranch {
    fn from(branch: git2::BranchType) -> Self {
        match branch {
            git2::BranchType::Local => GitBranch::Local,
            git2::BranchType::Remote => GitBranch::Remote,
        }
    }
}

mod reference;
pub use reference::GitReference;

mod repo_iter;
pub use repo_iter::GitRepositories;

mod repo;
pub use repo::GitRepo;

mod status_entry;
pub use status_entry::{GitStatusEntry, FileStatus};

mod status_iter;
pub use status_iter::GitStatusIter;

mod statuses;
pub use statuses::GitStatuses;

mod manifest;
pub use manifest::{Manifest, ManifestError};

mod manifest_iter;
pub use manifest_iter::ManifestIterator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
