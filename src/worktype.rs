use gitlib::{GitRepo, Status};
use std::{path::PathBuf, sync::mpsc::Sender};

pub enum WorkResult {
    Branch {
        path: PathBuf,
        branch: String,
        option: BranchOption,
    },
    Checkout {
        path: PathBuf,
        branch: String,
    },
    Reset {
        path: PathBuf,
        head: String,
    },
    Status {
        path: PathBuf,
        statuses: Vec<(PathBuf, Status)>,
    },
}

pub enum BranchOption {
    Delete,
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
    pub fn branch(index: usize, path: PathBuf, branch: String, option: BranchOption) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Branch {
                path,
                branch,
                option,
            },
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

    pub fn reset(index: usize, path: PathBuf, head: String) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Reset { path, head },
        }
    }

    pub fn status(index: usize, path: PathBuf, statuses: Vec<(PathBuf, Status)>) -> Self {
        WorkType::Work {
            index,
            message: WorkResult::Status { path, statuses },
        }
    }
}
