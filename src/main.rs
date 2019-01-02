mod cli;
mod dispatcher;

use crate::dispatcher::Dispatcher;
use color_printer::ColorPrinter;
use command::WorkType;
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
};
use threadpool::ThreadPool;
use util::RepoIter;

fn main() {
    let (command, working_path) = cli::handle_args().destructure();

    let is_terminal = atty::is(atty::Stream::Stdout);
    let stream = color_printer::StandardStream::stdout(color_printer::ColorChoice::Auto);
    let printer = ColorPrinter::new(is_terminal, &stream);

    let pool = threadpool::Builder::new().build();
    let rx = start_repo_iter(working_path, &pool);

    let mut dispatcher = Dispatcher::new(&pool, printer, command);
    dispatcher.run(&rx);
}

fn start_repo_iter(working_dir: PathBuf, pool: &ThreadPool) -> Receiver<WorkType> {
    let (tx, rx) = channel();
    let tx_send = tx.clone();

    pool.execute(move || {
        for (index, repo) in RepoIter::new(working_dir).enumerate() {
            tx.send(WorkType::repo(index, repo, tx_send.clone()))
                .expect("Could not signal main thread with WorkType::Repo");
        }
    });

    rx
}
