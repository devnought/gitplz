use crate::worktype::WorkResult;
use gitlib::GitRepo;

pub trait CommandBoxClone {
    fn box_clone(&self) -> Box<dyn Command>;
}

pub trait Command: Send + CommandBoxClone {
    fn process(&self, repo: GitRepo) -> WorkOption;
}

pub type WorkOption = Option<Box<dyn WorkResult>>;
