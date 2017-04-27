#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gitlib;

mod manifest;
pub use manifest::{Manifest, ManifestError};

mod manifest_iter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
