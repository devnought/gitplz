use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};

use term_painter::Color::{BrightCyan, BrightGreen, BrightMagenta, BrightRed};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use gitlib::{FileStatus, GitRepo, GitStatusEntry};
use util::GitRepositories;

const THREAD_SIGNAL: &str = "Could not signal main thread";

pub struct StatusData<T> {
    path: PathBuf,
    payload: T,
    index: usize,
}

pub enum StatusResult<T> {
    Empty(usize),
    Data(StatusData<T>),
}

pub fn process_repositories(
    repos: GitRepositories,
    pool: &ThreadPool,
    action: fn(tx: Sender<StatusResult<Vec<GitStatusEntry>>>, index: usize, repo: GitRepo),
) {
    let rx = execute_on_thread(repos, pool, action);

    let mut queue = BTreeMap::new();
    let mut next_index = 0;

    while let Ok(result) = rx.recv() {
        let data = match result {
            StatusResult::Data(d) => d,
            StatusResult::Empty(i) => {
                if i == next_index {
                    next_index = process_queue(&mut queue, next_index + 1);
                } else {
                    queue.insert(i, None);
                }
                continue;
            }
        };

        if next_index != data.index {
            queue.insert(data.index, Some((data.path, data.payload)));
            continue;
        }

        // TODO: Make this work
        //print_status(&data.path, data.payload);

        // If there are adjacent items in the queue, process them.
        next_index = process_queue(&mut queue, next_index + 1);
    }

    if !queue.is_empty() {
        panic!("Queue somehow has unprocessed items");
    }
}

fn process_queue(
    queue: &mut BTreeMap<usize, Option<(PathBuf, Vec<GitStatusEntry>)>>,
    index: usize,
) -> usize {
    let mut next_index = index;

    while let Some(opt) = queue.remove(&next_index) {
        if let Some((path, list)) = opt {
            // TODO: Make this work
            //print_status(&path, list);
        }

        next_index += 1;
    }

    next_index
}

fn execute_on_thread(
    repos: GitRepositories,
    pool: &ThreadPool,
    f: fn(Sender<StatusResult<Vec<GitStatusEntry>>>, usize, GitRepo),
) -> Receiver<StatusResult<Vec<GitStatusEntry>>> {
    let (tx, rx) = channel();

    for (index, repo) in repos.enumerate() {
        let tx = tx.clone();

        pool.execute(move || {
            f(tx, index, repo);
        });
    }

    rx
}
