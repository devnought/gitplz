#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gitlib;

use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder};
use std::io::Write;

use gitlib::{GitRepo, GitRepositories};

#[derive(Debug)]
pub struct Manifest {
    data: ManifestData,
    file: File,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManifestData {
    repos: Vec<String>,
}

impl ManifestData {
    fn empty() -> Self {
        ManifestData { repos: Vec::new() }
    }

    fn add(&mut self, repo: &GitRepo) {
        let path = String::from(repo.path().to_str().unwrap());
        self.repos.push(path);
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

        let (manifest_data, file) = match path_ref.exists() {
            true => {
                let file = File::open(path_ref).unwrap();

                match serde_json::from_reader(&file) {
                    Ok(m) => (m, file),
                    Err(e) => (ManifestData::empty(), file),
                }
            }
            false => {
                DirBuilder::new()
                    .recursive(true)
                    .create(path_ref.parent().unwrap())
                    .map_err(|_| ManifestError::BuildPath)?;

                let mut file = File::create(path_ref).unwrap();
                let data = ManifestData::empty();

                (data, file)
            }
        };

        Ok(Manifest {
               data: manifest_data,
               file: file,
           })
    }

    pub fn add_repositories(&mut self, repos: GitRepositories) {
        for repo in repos {
            self.data.add(&repo);
        }

        let ser_data = serde_json::to_string_pretty(&self.data).unwrap();
        println!("{}", &ser_data);

        write!(self.file, "{}", ser_data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
