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

mod repo;
pub use repo::GitRepo;

mod status_entry;
pub use status_entry::{GitStatusEntry, FileStatus};

mod status_iter;
pub use status_iter::GitStatusIter;

mod statuses;
pub use statuses::GitStatuses;

mod reference;
pub use reference::GitReference;