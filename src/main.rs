#![warn(rust_2018_idioms)]

#[macro_use]
extern crate structopt;

mod cli;
mod dispatcher;

use color_printer::ColorPrinter;
use command::*;
use crate::{cli::RunOption, dispatcher::Dispatcher};
use std::{
    env,
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
};
use threadpool::ThreadPool;
use util::RepoIter;

fn main() {
    let run_option = cli::handle_args();
    let working_path = run_option
        .path()
        .map(|x| x.into())
        .unwrap_or_else(|| env::current_dir().expect("Could not get working directory"));

    let is_terminal = atty::is(atty::Stream::Stdout);
    let stream = color_printer::StandardStream::stdout(color_printer::ColorChoice::Auto);
    let printer = ColorPrinter::new(is_terminal, &stream);

    let command: Box<dyn Command> = match run_option {
        RunOption::Branch { delete, find, .. } => {
            // TODO: This needs to be an enum again
            if let Some(branch) = delete {
                Box::new(BranchDeleteCommand::new(branch))
            } else if let Some(branch) = find {
                Box::new(BranchFindCommand::new(branch))
            } else {
                panic!("Invalid branch option");
            }
        }
        RunOption::Checkout { branch, .. } => Box::new(CheckoutCommand::new(branch)),
        RunOption::Reset { .. } => Box::new(ResetCommand::new()),
        RunOption::Status { .. } => Box::new(StatusCommand::new()),
    };

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
