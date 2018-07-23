use std::{
    fs::{DirEntry, ReadDir},
    io,
    iter::Filter,
    path::PathBuf,
};

crate type InternalRepoIter = Filter<ReadDir, fn(&io::Result<DirEntry>) -> bool>;

crate struct RepoIterState {
    pending: Vec<PathBuf>,
    filtered: Option<InternalRepoIter>,
}

impl RepoIterState {
    crate fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            pending: vec![path.into()],
            filtered: None,
        }
    }

    crate fn get_iter(&mut self) -> Option<&mut InternalRepoIter> {
        if self.filtered.is_some() {
            return self.filtered.as_mut();
        }

        let iter: InternalRepoIter = loop {
            let root = match self.pending.pop() {
                None => return None,
                Some(root) => root,
            };

            match root.read_dir() {
                Err(_) => continue,
                Ok(iter) => break iter.filter(filter_entry),
            }
        };

        self.filtered = Some(iter);
        self.filtered.as_mut()
    }

    crate fn end_iter(&mut self) {
        self.filtered = None;
    }

    crate fn add_pending(&mut self, entry: PathBuf) {
        self.pending.push(entry);
    }
}

fn filter_entry(entry: &io::Result<DirEntry>) -> bool {
    if entry.is_err() {
        return false;
    }

    let entry = entry.as_ref().unwrap();

    !is_file(entry) && !is_hidden(entry)
}

fn is_file(entry: &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_file()
    } else {
        false
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
