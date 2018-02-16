use gitlib::{GitError, GitRepo};

use std::fs::ReadDir;
use std::path::{Path, PathBuf};

struct ExploratoryMode {
    read_dir: Option<ReadDir>,
    pending: Vec<PathBuf>,
}

// TODO: This iterator is a mess because I didn't want to box
//       the previous version using map and filter functions.
//       This can probably go back to what it was once
//       impl Trait lands.
impl Iterator for ExploratoryMode {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.pending.is_empty() || self.read_dir.is_some() {
            {
                let iter = match self.read_dir {
                    Some(ref mut it) => it,
                    None => {
                        let current_dir = match self.pending.pop() {
                            Some(p) => p,
                            None => continue,
                        };

                        let read_iterator = match current_dir.read_dir() {
                            Ok(r) => r,
                            Err(_) => continue,
                        };

                        self.read_dir = Some(read_iterator);
                        self.read_dir
                            .as_mut()
                            .expect("This should have never failed")
                    }
                };

                for dir_entry in iter {
                    let entry = match dir_entry {
                        Ok(d) => d,
                        Err(_) => continue,
                    };

                    match entry.file_type() {
                        Ok(t) => {
                            if !t.is_dir() {
                                continue;
                            }
                        }
                        Err(_) => continue,
                    }

                    let path = entry.path();

                    match path.file_name() {
                        Some(name) => match name.to_str() {
                            Some(name_str) => {
                                if name_str.starts_with('.') || name_str.starts_with('$') {
                                    continue;
                                }
                            }
                            None => continue,
                        },
                        None => continue,
                    };

                    let repo = match GitRepo::new(&path) {
                        Ok(r) => r,
                        Err(GitError::OpenRepo) => {
                            self.pending.push(path.to_path_buf());
                            continue;
                        }
                        Err(_) => continue,
                    };

                    return Some(repo);
                }
            }

            self.read_dir = None;
        }

        None
    }
}

pub struct GitRepositories {
    mode: ExploratoryMode,
}

impl GitRepositories {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let path_ref = path.as_ref();
        let exp = ExploratoryMode {
            read_dir: None,
            pending: vec![path_ref.to_owned()],
        };

        Self { mode: exp }
    }
}

impl Iterator for GitRepositories {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        self.mode.next()
    }
}
