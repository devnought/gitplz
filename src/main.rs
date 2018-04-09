extern crate atty;
#[macro_use]
extern crate clap;
extern crate gitlib;
extern crate termcolor;
extern crate threadpool;
extern crate util;

use std::{env, sync::mpsc::{channel, Receiver}};

use util::RepoIter;
use threadpool::ThreadPool;

mod cli;
use cli::CommandArg;

mod worktype;
use worktype::WorkType;

mod dispatcher;
use dispatcher::Dispatcher;

mod process;
use process::Processor;

mod printer;
use printer::Printer;

mod printopts;
use printopts::PrintOptions;

fn main() {
    let run_option = {
        match cli::handle_args() {
            CommandArg::Help => {
                cli::print_help();
                return;
            }
            CommandArg::Completions { shell } => {
                cli::gen_completions_for(shell);
                return;
            }
            CommandArg::Run { option } => option,
        }
    };

    let pool = threadpool::Builder::new().build();
    let rx = start_repo_iter(&pool);

    let print_opts = PrintOptions::new(atty::is(atty::Stream::Stdout));
    let printer = Printer::new(&print_opts);
    let processor = Processor::new(&pool, &run_option);
    let mut dispatcher = Dispatcher::new(&processor, &printer);

    // Main dispatch processor. All threadpool messages get processed here.
    dispatcher.run(&rx);
}

fn start_repo_iter(pool: &ThreadPool) -> Receiver<WorkType> {
    let working_dir = env::current_dir().expect("Could not get working directory");
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
