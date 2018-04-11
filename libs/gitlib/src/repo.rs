use super::{Error, Reference, Statuses, git2};
use std::path::{Path, PathBuf};

pub struct GitRepo {
    path: PathBuf,
    repo: git2::Repository,
}

unsafe impl Send for GitRepo {}

impl GitRepo {
    pub fn open<P>(path: P) -> Result<Self, Error>
    where
        P: Into<PathBuf>,
        P: AsRef<Path>,
    {
        let owned_path = path.into();
        let git_repo = git2::Repository::open(&owned_path)?;

        let repo = Self {
            path: owned_path,
            repo: git_repo,
        };

        Ok(repo)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn statuses(&self) -> Result<Statuses, Error> {
        let mut opts = git2::StatusOptions::new();

        opts.include_ignored(false)
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_unreadable_as_untracked(true)
            .disable_pathspec_match(true)
            .exclude_submodules(true);

        self.repo
            .statuses(Some(&mut opts))
            .map(|x| x.into())
            .map_err(|x| x.into())
    }

    pub fn reset(&self) -> Result<Reference, Error> {
        let head = self.repo.head()?;
        let obj = head.peel(git2::ObjectType::Any)?;

        self.repo.reset(&obj, git2::ResetType::Hard, None)?;

        Reference::new(&head)
    }

    pub fn checkout(&self, branch_name: &str) -> Result<bool, Error> {
        let branch_type = if branch_name.starts_with("origin/") {
            git2::BranchType::Remote
        } else {
            git2::BranchType::Local
        };

        let branch = self.repo.find_branch(branch_name, branch_type)?;

        let obj = branch.get().peel(git2::ObjectType::Commit)?;

        match branch_type {
            git2::BranchType::Local => self.checkout_local(branch_name, &obj),
            git2::BranchType::Remote => self.checkout_remote(&obj),
        }
    }

    fn checkout_local(&self, branch_name: &str, obj: &git2::Object) -> Result<bool, Error> {
        let mut opts = git2::build::CheckoutBuilder::new();

        self.repo.checkout_tree(obj, Some(&mut opts))?;

        let branch_str = format!("refs/heads/{}", branch_name);
        let branch_ref = self.repo.find_reference(&branch_str)?;

        self.repo
            .set_head(branch_ref.name().unwrap())
            .expect("Error setting head");

        Ok(true)
    }

    fn checkout_remote(&self, obj: &git2::Object) -> Result<bool, Error> {
        let head_id = self.repo
            .head()
            .expect("Could not resolve head")
            .peel(git2::ObjectType::Any)
            .expect("Could not get head ref")
            .id();

        if head_id == obj.id() {
            //println!("bailing out");
            return Ok(false);
        }

        self.repo.set_head_detached(obj.id()).expect("wut");
        self.repo
            .reset(obj, git2::ResetType::Hard, None)
            .expect("wuufttttt");

        Ok(true)
    }

    pub fn delete_local_branch(&self, branch_name: &str) -> Result<bool, Error> {
        self.repo
            .find_branch(branch_name, git2::BranchType::Local)?
            .delete()?;

        Ok(true)
    }

    pub fn has_local_branch(&self, branch_name: &str) -> Result<bool, Error> {
        self.repo.find_branch(branch_name, git2::BranchType::Local)?;

        Ok(true)
    }
}
