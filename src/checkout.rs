use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver};
use gitlib::GitError;

use term_painter::Color::{BrightCyan, BrightWhite};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use gitlib::GitStatusEntry;
use util::GitRepositories;

const THREAD_SIGNAL: &str = "Could not signal main thread";

pub fn process_checkout(
    repos: GitRepositories,
    branch: &str,
    pool: &ThreadPool,
) -> Result<(i32), GitError> {
    let mut branches = 0;

    for repo in repos {
        match repo.checkout(branch) {
            Ok(true) => branches += 1,
            Ok(false) => continue,
            Err(_) => {
                //println!("{}: Error checking out branch '{}': {:?}", repo.path().display(), branch, e);
                continue;
            }
        }

        println!(" {}", BrightWhite.paint(repo.path().display()));
        println!("     {}", BrightCyan.paint(branch));
    }

    Ok(branches)
}
/*
fn checkout(repos: GitRepositories, branch: &str, pool: &ThreadPool) -> Receiver<usize> {
    for repo in repos {
        match repo.checkout(branch) {
            Ok(true) => branches += 1,
            Ok(false) => continue,
            Err(_) => {
                //println!("{}: Error checking out branch '{}': {:?}", repo.path().display(), branch, e);
                continue;
            }
        }

        println!(" {}", BrightWhite.paint(repo.path().display()));
        println!("     {}", BrightCyan.paint(branch));
    }
}
*/
