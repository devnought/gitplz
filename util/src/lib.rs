#![feature(rust_2018_preview)]

mod repo_iter;
mod repo_iter_state;
pub use crate::repo_iter::RepoIter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
