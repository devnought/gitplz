extern crate git2;

#[derive(Debug)]
pub enum GitError {
    Checkout(GitBranch),
    Manifest,
    OpenRepo,
    RemoveUntracked,
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

mod repo;
pub use repo::GitRepo;

mod status_entry;
pub use status_entry::{FileStatus, GitStatusEntry};

mod status_iter;
pub use status_iter::GitStatusIter;

mod statuses;
pub use statuses::GitStatuses;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
