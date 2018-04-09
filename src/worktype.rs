use gitlib::{GitRepo, Status};
use std::{path::PathBuf, sync::mpsc::Sender};

pub enum WorkResult {
    Status {
        path: PathBuf,
        statuses: Vec<(PathBuf, Status)>,
    },
    Reset {
        path: PathBuf,
        head: String,
    },
    Checkout {
        path: PathBuf,
        branch: String,
    },
}

pub enum WorkType {
    Repo {
        index: usize,
        repo: GitRepo,
        tx: Sender<WorkType>,
    },
    Work {
        index: usize,
        message: WorkResult,
    },
    WorkEmpty {
        index: usize,
    },
}

impl WorkType {
    pub fn status(index: usize, path: PathBuf, statuses: Vec<(PathBuf, Status)>) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Status { path, statuses },
        }
    }

    pub fn reset(index: usize, path: PathBuf, head: String) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Reset { path, head },
        }
    }

    pub fn checkout(index: usize, path: PathBuf, branch: String) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Checkout { path, branch },
        }
    }

    pub fn empty(index: usize) -> Self {
        WorkType::WorkEmpty { index }
    }

    pub fn repo(index: usize, repo: GitRepo, tx: Sender<WorkType>) -> Self {
        WorkType::Repo { index, repo, tx }
    }
}
