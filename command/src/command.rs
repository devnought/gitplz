use gitlib::GitRepo;
use worktype::WorkType;

pub trait CommandBoxClone {
    fn box_clone(&self) -> Box<Command>;
}

pub trait Command: Send + CommandBoxClone {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType;
}
