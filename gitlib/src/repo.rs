use crate::{Error, Reference, Statuses};
use git2;
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

    pub fn statuses(&self) -> Result<Statuses<'_>, Error> {
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

        Reference::from_ref(&head)
    }

    pub fn checkout(&self, branch_name: &str) -> Result<bool, Error> {
        let branch_type = self.get_branch_type(branch_name)?;
        let branch = self.repo.find_branch(branch_name, branch_type)?;
        let obj = branch.get().peel(git2::ObjectType::Commit)?;

        match branch_type {
            git2::BranchType::Local => self.checkout_local(branch_name, &obj),
            git2::BranchType::Remote => self.checkout_remote(&obj),
        }
    }

    pub fn delete_local_branch(&self, branch_name: &str) -> Result<(), Error> {
        self.repo
            .find_branch(branch_name, git2::BranchType::Local)?
            .delete()?;

        Ok(())
    }

    pub fn fetch(&self) -> Result<(), Error> {
        let refspecs = self.repo.find_remote("origin")?.fetch_refspecs()?;
        let refspec_collection = refspecs.iter().filter_map(|x| x).collect::<Vec<_>>();

        let mut fetch_options = {
            let mut remote_callbacks = git2::RemoteCallbacks::new();
            remote_callbacks.credentials(Self::credentials_callback);

            let mut o = git2::FetchOptions::new();
            o.remote_callbacks(remote_callbacks);
            o
        };

        // TODO: Instead of refspec_collection, maybe the following:
        // &["refs/heads/*:refs/heads/*"]
        // Example here: https://github.com/rust-lang/crates.io/blob/master/src/git.rs#L114-L209

        if let Err(e) =
            self.repo
                .find_remote("origin")?
                .fetch(&refspec_collection, Some(&mut fetch_options), None)
        {
            let asd = format!("{:?}", e);
            return Err(Error::GenericError);
        }

        Ok(())
    }

    pub fn has_local_branch(&self, branch_name: &str) -> Result<(), Error> {
        self.repo
            .find_branch(branch_name, git2::BranchType::Local)?;

        Ok(())
    }

    fn get_branch_type(&self, branch_name: &str) -> Result<git2::BranchType, Error> {
        let components = branch_name.split('/').collect::<Vec<_>>();

        match components.len() {
            0 => Err(Error::ZeroSizedBranchName),
            1 => Ok(git2::BranchType::Local),
            _ => {
                if self.repo.find_remote(&components[0]).is_ok() {
                    Ok(git2::BranchType::Remote)
                } else {
                    Ok(git2::BranchType::Local)
                }
            }
        }
    }

    fn checkout_local(&self, branch_name: &str, obj: &git2::Object<'_>) -> Result<bool, Error> {
        self.repo.checkout_tree(obj, None)?;

        let branch_str = format!("refs/heads/{}", branch_name);
        let branch_ref = self.repo.find_reference(&branch_str)?;

        self.repo
            .set_head(branch_ref.name().ok_or(Error::GenericError)?)?;

        Ok(true)
    }

    fn checkout_remote(&self, obj: &git2::Object<'_>) -> Result<bool, Error> {
        let head_id = self.repo.head()?.peel(git2::ObjectType::Any)?.id();

        if head_id == obj.id() {
            return Ok(false);
        }

        self.repo.set_head_detached(obj.id())?;
        self.repo.reset(obj, git2::ResetType::Hard, None)?;

        Ok(true)
    }

    fn credentials_callback(
        _user: &str,
        _user_from_url: Option<&str>,
        _cred: git2::CredentialType,
    ) -> Result<git2::Cred, git2::Error> {
        git2::Cred::ssh_key_from_agent("kgretchev")
    }
}
