use std::path::Path;
use super::{git2, GitStatuses, GitError, GitReference};

pub struct GitRepo {
    repo: git2::Repository,
}

impl GitRepo {
    pub fn new(path: &Path) -> Result<Self, GitError> {
        let repo = git2::Repository::open(path).map_err(|_| GitError::OpenRepo)?;

        Ok(GitRepo { repo: repo })
    }

    pub fn statuses(&self) -> Result<GitStatuses, GitError> {
        let mut opts = git2::StatusOptions::new();

        opts.include_ignored(false)
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_unreadable_as_untracked(true)
            .disable_pathspec_match(true)
            .exclude_submodules(true);

        let statuses = self.repo
            .statuses(Some(&mut opts))
            .map_err(|_| GitError::Status)?;

        Ok(GitStatuses::new(statuses))
    }

    pub fn reset(&self) -> Result<GitReference, GitError> {
        let head = self.repo.head().map_err(|_| GitError::Reset)?;
        let obj = head.peel(git2::ObjectType::Any)
            .map_err(|_| GitError::Reset)?;

        let mut builder = git2::build::CheckoutBuilder::new();
        let mut options = builder
            .remove_untracked(true)
            .progress(|path, a, b| {
                if path == None {
                    return;
                }

                println!("{:?} {:?} {:?}", path, a, b)
            });

        self.repo
            .reset(&obj, git2::ResetType::Hard, Some(options))
            .map_err(|_| GitError::Reset)?;

        Ok(GitReference::new(head))
    }

    pub fn checkout(&self, branch: &str) -> Result<(), GitError> {
        Ok(())
    }
}