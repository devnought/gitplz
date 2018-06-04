use gitlib::GitRepo;
use worktype::WorkType;

pub trait Command: Send {
    fn process(&self, index: usize, repo: GitRepo) -> WorkType;
    fn box_clone(&self) -> Box<Command>;
}
