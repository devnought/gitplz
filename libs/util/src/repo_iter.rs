use gitlib::{GitRepo, GitError};
use Manifest;

use std::fs::ReadDir;
use std::path::{Path, PathBuf};

struct ExploratoryMode {
    read_dir: Option<ReadDir>,
    pending: Vec<PathBuf>,
}

impl Iterator for ExploratoryMode {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pending.len() > 0 || self.read_dir.is_some() {
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

struct ManifestMode {}

impl Iterator for ManifestMode {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

enum RepoMode {
    Exploratory(ExploratoryMode),
    Manifest(ManifestMode),
}

pub struct GitRepositories {
    mode: RepoMode,
}

impl GitRepositories {
    pub fn new(path: &Path) -> Self {
        let exp = ExploratoryMode {
            read_dir: None,
            pending: vec![path.to_owned()],
        };

        GitRepositories { mode: RepoMode::Exploratory(exp) }
    }

    pub fn from_manifest(manifest: &Manifest) -> Self {
        let man = ManifestMode {};

        GitRepositories { mode: RepoMode::Manifest(man) }
    }
}

impl Iterator for GitRepositories {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            RepoMode::Exploratory(ref mut em) => em.next(),
            RepoMode::Manifest(ref mut mm) => mm.next(),
        }
    }
}