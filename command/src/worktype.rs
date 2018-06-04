use color_printer::ColorPrinter;
use gitlib::GitRepo;
use std::{marker::Send, sync::mpsc::Sender};

pub trait WorkResult: Send {
    fn print(&self, printer: &mut ColorPrinter);
}

pub enum WorkType {
    Repo {
        index: usize,
        repo: GitRepo,
        tx: Sender<WorkType>,
    },
    Work {
        index: usize,
        result: Box<WorkResult>,
    },
    WorkEmpty {
        index: usize,
    },
}

impl WorkType {
    pub fn result(index: usize, result: Box<WorkResult>) -> Self {
        WorkType::Work { index, result }
    }

    pub fn empty(index: usize) -> Self {
        WorkType::WorkEmpty { index }
    }

    pub fn repo(index: usize, repo: GitRepo, tx: Sender<WorkType>) -> Self {
        WorkType::Repo { index, repo, tx }
    }
}
