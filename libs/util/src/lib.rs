#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gitlib;

mod manifest;
pub use manifest::{Manifest, ManifestError};

mod manifest_iter;
pub use manifest_iter::ManifestIterator;

mod repo_iter;
pub use repo_iter::GitRepositories;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
