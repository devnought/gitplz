use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};

use term_painter::Color::{BrightCyan, BrightGreen, BrightMagenta, BrightRed};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use gitlib::{FileStatus, GitRepo, GitStatusEntry};
use util::GitRepositories;

const THREAD_SIGNAL: &str = "Could not signal main thread";

struct StatusData<T> {
    path: PathBuf,
    payload: T,
    index: usize,
}

enum StatusResult<T> {
    Empty(usize),
    Data(StatusData<T>),
}

fn whatever_process(tx: Sender<StatusResult<Vec<GitStatusEntry>>>, index: usize, repo: GitRepo) {
    let statuses = match repo.statuses() {
        Ok(s) => s,
        Err(_) => {
            tx.send(StatusResult::Empty(index)).expect(THREAD_SIGNAL);
            return;
        }
    };

    if statuses.is_empty() {
        tx.send(StatusResult::Empty(index)).expect(THREAD_SIGNAL);
        return;
    }

    let statuses_result = statuses.iter().collect::<Vec<_>>();
    let data = StatusData {
        path: repo.path().to_path_buf(),
        payload: statuses_result,
        index: index,
    };

    tx.send(StatusResult::Data(data)).expect(THREAD_SIGNAL);
}

fn print_status(path: &Path, list: Vec<GitStatusEntry>) {
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

        println!(" {} {}", colour.paint(pre), entry.path().display());
    }
}

pub fn process_status(repos: GitRepositories, pool: &ThreadPool) {
    let rx = repo_status(repos, pool, whatever_process);

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

        print_status(&data.path, data.payload);

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
            print_status(&path, list);
        }

        next_index += 1;
    }

    next_index
}

fn repo_status(
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
