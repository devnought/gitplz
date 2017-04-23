#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gitlib;

use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder};
use std::io::Write;
use std::collections::HashSet;

use gitlib::{GitRepo, GitRepositories};

#[derive(Debug)]
pub struct Manifest {
    data: ManifestData,
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManifestData {
    repos: HashSet<String>,
}

impl ManifestData {
    fn empty() -> Self {
        ManifestData { repos: HashSet::new() }
    }

    fn add(&mut self, repo: &GitRepo) {
        let path = String::from(repo.path().to_str().unwrap());
        self.repos.insert(path);
    }
}

#[derive(Debug)]
pub enum ManifestError {
    BuildPath,
    PathNotFound,
}

impl Manifest {
    pub fn open<P>(path: P) -> Result<Self, ManifestError>
        where P: AsRef<Path>
    {
        let path_ref = path.as_ref();

        let manifest_data = match path_ref.exists() {
            true => {
                let file = File::open(path_ref).unwrap();

                match serde_json::from_reader(&file) {
                    Ok(m) => m,
                    Err(e) => ManifestData::empty(),
                }
            }
            false => ManifestData::empty(),
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
        println!("{}", &ser_data);

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
