use gitlib::{GitRepo, Status};
use cli::{self, RunOption};
use threadpool::ThreadPool;

use std::{fs, sync::mpsc::Sender};

use worktype::{BranchOption, WorkType};

const THREAD_SIGNAL: &str = "Could not signal main thread with WorkType::Work";

pub struct Processor<'a> {
    pool: &'a ThreadPool,
    run_option: &'a RunOption,
}

impl<'a> Processor<'a> {
    pub fn new(pool: &'a ThreadPool, run_option: &'a RunOption) -> Self {
        Self { pool, run_option }
    }

    pub fn repo(&self, tx: Sender<WorkType>, index: usize, repo: GitRepo) {
        match *self.run_option {
            RunOption::Branch {
                ref branch,
                ref option,
            } => {
                let branch = branch.clone();
                match *option {
                    cli::BranchOption::Delete => self.pool.execute(move || {
                        tx.send(Self::branch_delete(&repo, index, branch))
                            .expect(THREAD_SIGNAL)
                    }),
                }
            }
            RunOption::Checkout { ref branch } => {
                let branch = branch.clone();
                self.pool.execute(move || {
                    tx.send(Self::checkout(&repo, index, branch))
                        .expect(THREAD_SIGNAL)
                });
            }
            RunOption::Reset => self.pool
                .execute(move || tx.send(Self::reset(&repo, index)).expect(THREAD_SIGNAL)),
            RunOption::Status => self.pool
                .execute(move || tx.send(Self::status(&repo, index)).expect(THREAD_SIGNAL)),
        }
    }

    fn status(repo: &GitRepo, index: usize) -> WorkType {
        let statuses = match repo.statuses() {
            Err(_) => return WorkType::empty(index),
            Ok(ref s) if s.is_empty() => return WorkType::empty(index),
            Ok(s) => s,
        };

        let mut results = Vec::new();

        for status_entry in statuses.iter() {
            for (path, status) in status_entry.iter() {
                results.push((path.to_owned(), status));
            }
        }

        WorkType::status(index, repo.path().into(), results)
    }

    fn reset(repo: &GitRepo, index: usize) -> WorkType {
        // If we can get the status of the repo, try that first
        // instead of blindly resetting when it's not required.
        let status_result = repo.statuses();

        let statuses = match status_result {
            Err(_) => None,
            Ok(s) => {
                if s.is_empty() {
                    return WorkType::empty(index);
                }

                Some(s)
            }
        };

        // Check for any 'new' files to delete
        if let Some(s) = statuses {
            let iter = s.iter()
                .filter(|x| {
                    for status in x.iter() {
                        if let (_, Status::New) = status {
                            return true;
                        }
                    }

                    false
                })
                .map(|x| repo.path().join(x.path()));

            for path in iter {
                fs::remove_file(path).expect("Could not remove file");
            }
        }

        // Proceed with normal reset
        let head = match repo.reset() {
            Err(_) => return WorkType::empty(index),
            Ok(h) => h,
        };

        WorkType::reset(index, repo.path().into(), head.name().into())
    }

    fn checkout(repo: &GitRepo, index: usize, branch: String) -> WorkType {
        if let Ok(true) = repo.checkout(&branch) {
            WorkType::checkout(index, repo.path().into(), branch)
        } else {
            WorkType::empty(index)
        }
    }

    fn branch_delete(repo: &GitRepo, index: usize, branch: String) -> WorkType {
        if let Ok(true) = repo.delete_local_branch(&branch) {
            WorkType::branch(index, repo.path().into(), branch, BranchOption::Delete)
        } else {
            WorkType::empty(index)
        }
    }
}
