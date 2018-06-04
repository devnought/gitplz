extern crate git2;

#[derive(Debug)]
pub enum Error {
    GenericError,
    NotFound,
    Exists,
    Ambiguous,
    BufSize,
    User,
    BareRepo,
    UnbornBranch,
    Unmerged,
    NotFastForward,
    InvalidSpec,
    Conflict,
    Locked,
    Modified,
    Auth,
    Certificate,
    Applied,
    Peel,
    Eof,
    Invalid,
    Uncommitted,
    Directory,
    InvalidUtf8,
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        match error.code() {
            git2::ErrorCode::GenericError => Error::GenericError,
            git2::ErrorCode::NotFound => Error::NotFound,
            git2::ErrorCode::Exists => Error::Exists,
            git2::ErrorCode::Ambiguous => Error::Ambiguous,
            git2::ErrorCode::BufSize => Error::BufSize,
            git2::ErrorCode::User => Error::User,
            git2::ErrorCode::BareRepo => Error::BareRepo,
            git2::ErrorCode::UnbornBranch => Error::UnbornBranch,
            git2::ErrorCode::Unmerged => Error::Unmerged,
            git2::ErrorCode::NotFastForward => Error::NotFastForward,
            git2::ErrorCode::InvalidSpec => Error::InvalidSpec,
            git2::ErrorCode::Conflict => Error::Conflict,
            git2::ErrorCode::Locked => Error::Locked,
            git2::ErrorCode::Modified => Error::Modified,
            git2::ErrorCode::Auth => Error::Auth,
            git2::ErrorCode::Certificate => Error::Certificate,
            git2::ErrorCode::Applied => Error::Applied,
            git2::ErrorCode::Peel => Error::Peel,
            git2::ErrorCode::Eof => Error::Eof,
            git2::ErrorCode::Invalid => Error::Invalid,
            git2::ErrorCode::Uncommitted => Error::Uncommitted,
            git2::ErrorCode::Directory => Error::Directory,
        }
    }
}

mod reference;
pub use reference::Reference;

mod repo;
pub use repo::GitRepo;

mod status_entry;
pub use status_entry::StatusEntry;

mod statuses;
pub use statuses::{StatusIter, Statuses};

mod status_entry_iter;
pub use status_entry_iter::{Status, StatusEntryIter};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
