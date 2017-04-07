extern crate git2;

#[derive(Debug)]
pub enum GitError {
    OpenRepo,
    Status,
    Reset,
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