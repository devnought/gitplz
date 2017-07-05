use gitlib::{GitRepo, GitError};
use Manifest;
use ManifestIterator;

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
        while self.pending.len() > 0 || self.read_dir.is_some() {
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

                while let Some(dir_entry) = iter.next() {
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

struct ManifestMode<'a> {
    iter: ManifestIterator<'a>,
}

impl<'a> Iterator for ManifestMode<'a> {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

enum RepoMode<'a> {
    Exploratory(ExploratoryMode),
    Manifest(ManifestMode<'a>),
}

pub struct GitRepositories<'a> {
    mode: RepoMode<'a>,
}

impl<'a> GitRepositories<'a> {
    pub fn new<P>(path: P) -> Self
        where P: AsRef<Path>
    {
        let path_ref = path.as_ref();
        let exp = ExploratoryMode {
            read_dir: None,
            pending: vec![path_ref.to_owned()],
        };

        Self { mode: RepoMode::Exploratory(exp) }
    }

    pub fn from_manifest(manifest: &'a Manifest) -> Self {
        let man = ManifestMode { iter: manifest.repos() };

        Self { mode: RepoMode::Manifest(man) }
    }
}

impl<'a> Iterator for GitRepositories<'a> {
    type Item = GitRepo;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            RepoMode::Exploratory(ref mut em) => em.next(),
            RepoMode::Manifest(ref mut mm) => mm.next(),
        }
    }
}
