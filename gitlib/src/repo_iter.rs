use super::{GitRepo, GitError};

use std::fs::ReadDir;
use std::path::{Path, PathBuf};

pub struct GitRepositories {
    read_dir: Option<ReadDir>,
    pending: Vec<PathBuf>,
}

impl GitRepositories {
    pub fn new(path: &Path) -> Self {
        GitRepositories {
            read_dir: None,
            pending: vec![path.to_owned()],
        }
    }
}

impl Iterator for GitRepositories {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pending.len() > 0 {
            {
                let iter = match self.read_dir {
                    Some(ref mut it) => it,
                    None => {
                        let current_dir = match self.pending.pop() {
                            Some(p) => p,
                            None => continue,
                        };

                        let read_result = current_dir.read_dir();

                        if let Err(_) = read_result {
                            continue;
                        }

                        let it = read_result.unwrap();
                        self.read_dir = Some(it);
                        self.read_dir.as_mut().unwrap()
                    }
                };

                while let Some(dir_entry) = iter.next() {
                    let entry = {
                        if !dir_entry.is_ok() {
                            continue;
                        }

                        dir_entry.unwrap()
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
                        Some(name) => {
                            match name.to_str() {
                                Some(name_str) => {
                                    if name_str.starts_with(".") || name_str.starts_with("$") {
                                        continue;
                                    }
                                }
                                None => continue,
                            }
                        }
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