extern crate gitlib;

mod repo_iter;
mod repo_iter_state;
pub use repo_iter::RepoIter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
