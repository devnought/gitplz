use serde_json;
use gitlib::{GitRepo, GitRepositories};
use manifest_iter::ManifestIterator;

use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder};
use std::io::Write;
use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestData {
    pub root: PathBuf,
    pub repos: BTreeSet<String>,
}

impl ManifestData {
    fn empty(path: &Path) -> Self {
        ManifestData {
            repos: BTreeSet::new(),
            root: path.to_path_buf(),
        }
    }

    fn add(&mut self, repo: &GitRepo) {
        let path_strip = repo.path().strip_prefix(&self.root).unwrap();
        let path = String::from(path_strip.to_str().unwrap());
        self.repos.insert(path);
    }
}

#[derive(Debug)]
pub enum ManifestError {
    BuildPath,
    PathNotFound,
}

#[derive(Debug)]
pub struct Manifest {
    data: ManifestData,
    path: PathBuf,
}

impl Manifest {
    pub fn open<P>(path: P, root: P) -> Result<Self, ManifestError>
        where P: AsRef<Path>
    {
        let path_ref = path.as_ref();

        let manifest_data = match path_ref.exists() {
            true => {
                let file = File::open(path_ref).unwrap();

                match serde_json::from_reader(&file) {
                    Ok(m) => m,
                    Err(_) => ManifestData::empty(root.as_ref()),
                }
            }
            false => ManifestData::empty(root.as_ref()),
        };

        Ok(Manifest {
               data: manifest_data,
               path: path_ref.to_path_buf(),
           })
    }

    pub fn add_repositories(&mut self, repos: GitRepositories) {
        for repo in repos {
            self.data.add(&repo);
        }

        let ser_data = serde_json::to_string_pretty(&self.data).unwrap();
        let mut file = self.get_file();

        match write!(file, "{}", ser_data) {
            Ok(_) => (),
            Err(e) => println!("{:#?}", e),
        }
    }

    fn get_file(&self) -> File {
        if !self.path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(self.path.parent().unwrap())
                .unwrap();
        }

        File::create(&self.path).unwrap()
    }

    pub fn paths(&self) -> ManifestIterator {
        ManifestIterator::new(&self.data)
    }
}