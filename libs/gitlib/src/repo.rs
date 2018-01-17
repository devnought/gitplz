use std::path::{Path, PathBuf};
use std::fs;

use super::{git2, GitStatuses, GitError, GitReference, GitBranch, FileStatus};

pub struct GitRepo {
    repo: git2::Repository,
    path: PathBuf,
}

unsafe impl Send for GitRepo {}
unsafe impl Sync for GitRepo {}

impl GitRepo {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, GitError> {
        let path_ref = path.as_ref();
        let repo = git2::Repository::open(path_ref)
            .map_err(|_| GitError::OpenRepo)?;

        Ok(Self {
               repo: repo,
               path: path_ref.to_owned(),
           })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn statuses(&self) -> Result<GitStatuses, GitError> {
        // self.repo.graph_ahead_behind

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
        let options = builder
            .remove_untracked(true) // this is ignored for a reset :()
            .progress(|path, a, b| {
                          if path == None {
                              return;
                          }

                          println!("{:?} {:?} {:?}", path, a, b)
                      });

        self.repo
            .reset(&obj, git2::ResetType::Hard, Some(options))
            .map_err(|_| GitError::Reset)?;

        Ok(GitReference::new(&head))
    }

    pub fn checkout(&self, branch_name: &str) -> Result<(), GitError> {
        let branch_type = match branch_name.find("origin/") {
            Some(_) => git2::BranchType::Remote,
            None => git2::BranchType::Local,
        };

        let branch = self.repo
            .find_branch(branch_name, branch_type)
            .map_err(|_| GitError::Checkout(GitBranch::from(branch_type)))?;

        let obj = branch
            .get()
            .peel(git2::ObjectType::Any)
            .map_err(|_| GitError::Checkout(GitBranch::from(branch_type)))?;

        let mut opts = git2::build::CheckoutBuilder::new();

        self.repo
            .checkout_tree(&obj, Some(&mut opts))
            .map_err(|_| GitError::Checkout(GitBranch::from(branch_type)))?;

        let branch_ref = match self.repo.find_reference("refs/heads/topic/STS-616") {
            Ok(r) => r,
            Err(e) => {
                println!("{:#?}", e);
                return Ok(());
            }
        };

        self.repo
            .set_head(branch_ref.name().unwrap())
            .expect("Error setting head");

        println!("  {} {}",
                 match branch_type {
                     git2::BranchType::Local => " [Local]",
                     git2::BranchType::Remote => "[Remote]",
                 },
                 branch_name);

        Ok(())
    }

    pub fn remove_untracked(&self) -> Result<(), GitError> {
        let statuses = self.statuses()?;
        let iter = statuses
            .iter()
            .filter(|x| match *x.status() {
                        FileStatus::New => true,
                        _ => false,
                    });

        // TODO: Finish this nonsense
        for entry in iter {
            let p = self.path.join(entry.path());

            // The whole file/directory distinction might be useless.
            // If a untracked file is removed from an untracked directory, should also
            // remove now empty directory?
            if p.is_file() {
                fs::remove_file(p)
                    .map_err(|_| GitError::RemoveUntracked)?;
            } else if p.is_dir() {
                fs::remove_dir_all(p)
                    .map_err(|_| GitError::RemoveUntracked)?;
            }
        }

        Ok(())
    }

    pub fn state(&self) -> RepoState {
        RepoState::from(self.repo.state())
    }
}

#[derive(Debug)]
pub enum RepoState {
    Clean,
    Merge,
    Revert,
    RevertSequence,
    CherryPick,
    CherryPickSequence,
    Bisect,
    Rebase,
    RebaseInteractive,
    RebaseMerge,
    ApplyMailbox,
    ApplyMailboxOrRebase,
}

impl From<git2::RepositoryState> for RepoState {
    fn from(state: git2::RepositoryState) -> Self {
        use git2::RepositoryState;

        match state {
            RepositoryState::Clean => RepoState::Clean,
            RepositoryState::Merge => RepoState::Merge,
            RepositoryState::Revert => RepoState::Revert,
            RepositoryState::RevertSequence => RepoState::RevertSequence,
            RepositoryState::CherryPick => RepoState::CherryPick,
            RepositoryState::CherryPickSequence => RepoState::CherryPickSequence,
            RepositoryState::Bisect => RepoState::Bisect,
            RepositoryState::Rebase => RepoState::Rebase,
            RepositoryState::RebaseInteractive => RepoState::RebaseInteractive,
            RepositoryState::RebaseMerge => RepoState::RebaseMerge,
            RepositoryState::ApplyMailbox => RepoState::ApplyMailbox,
            RepositoryState::ApplyMailboxOrRebase => RepoState::ApplyMailboxOrRebase,
        }
    }
}
