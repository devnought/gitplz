use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use gitlib::{FileStatus, GitStatusEntry};
use util::GitRepositories;

const THREAD_SIGNAL: &str = "Could not signal main thread";

enum StatusResult {
    Empty(usize),
    Data((PathBuf, Vec<GitStatusEntry>, usize)),
}

pub fn process_status(repos: GitRepositories, pool: ThreadPool) {
    let rx = repo_status(repos, pool);

    let mut queue = BTreeMap::new();
    let mut next_index = 0;

    while let Ok(result) = rx.recv() {
        let (path, list, index) = match result {
            StatusResult::Data(t) => t,
            StatusResult::Empty(i) => {
                if i == next_index {
                    next_index = process_queue(&mut queue, next_index + 1);
                } else {
                    queue.insert(i, None);
                }
                continue;
            }
        };

        if next_index != index {
            queue.insert(index, Some((path, list)));
            continue;
        }

        print_status(path, list);

        // If there are adjacent items in the queue, process them.
        next_index = process_queue(&mut queue, next_index + 1);
    }

    if !queue.is_empty() {
        panic!("Queue somehow has unprocessed items");
    }
}

fn process_queue(queue: &mut BTreeMap<usize, Option<(PathBuf, Vec<GitStatusEntry>)>>,
                 index: usize)
                 -> usize {
    let mut next_index = index;

    while let Some(opt) = queue.remove(&next_index) {
        if let Some((path, list)) = opt {
            print_status(path, list);
        }

        next_index += 1;
    }

    next_index
}

fn print_status(path: PathBuf, list: Vec<GitStatusEntry>) {
    println!("{}", path.display());

    for entry in list {
        let (pre, colour) = match *entry.status() {
            FileStatus::Conflicted => ("       Conflicted", BrightMagenta),
            FileStatus::Current => ("          Current", BrightMagenta),
            FileStatus::Deleted => ("          Deleted", BrightRed),
            FileStatus::Ignored => ("          Ignored", BrightMagenta),
            FileStatus::StagedNew => ("       Staged New", BrightMagenta),
            FileStatus::StagedModified => ("  Staged Modified", BrightMagenta),
            FileStatus::StagedDeleted => ("   Staged Deleted", BrightMagenta),
            FileStatus::StagedRenamed => ("   Staged Renamed", BrightMagenta),
            FileStatus::StagedTypechange => ("Staged Typechange", BrightMagenta),
            FileStatus::Modified => ("         Modified", BrightCyan),
            FileStatus::New => ("              New", BrightGreen),
            FileStatus::Renamed => ("          Renamed", BrightCyan),
            FileStatus::Typechange => ("       Typechange", BrightCyan),
            FileStatus::Unknown => ("          Unknown", BrightMagenta),
        };

        println!("  {} {}", colour.paint(pre), entry.path().display());
    }
}

fn repo_status(repos: GitRepositories, pool: ThreadPool) -> Receiver<StatusResult> {
    let (tx, rx) = channel();

    for (index, repo) in repos.enumerate() {
        let tx = tx.clone();

        pool.execute(move || {
            let statuses = match repo.statuses() {
                Ok(s) => s,
                Err(_) => {
                    tx.send(StatusResult::Empty(index)).expect(THREAD_SIGNAL);
                    return;
                }
            };

            if statuses.len() == 0 {
                tx.send(StatusResult::Empty(index)).expect(THREAD_SIGNAL);
                return;
            }

            let statuses_result = statuses.iter().collect::<Vec<_>>();
            let tuple = (repo.path().to_path_buf(), statuses_result, index);
            tx.send(StatusResult::Data(tuple)).expect(THREAD_SIGNAL);
        });
    }

    rx
}