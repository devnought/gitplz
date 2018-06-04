extern crate atty;
#[macro_use]
extern crate clap;
extern crate color_printer;
extern crate command;
extern crate gitlib;
extern crate threadpool;
extern crate util;

use std::{env,
          sync::mpsc::{channel, Receiver}};

use color_printer::ColorPrinter;
use threadpool::ThreadPool;
use util::RepoIter;

mod cli;
use cli::{BranchOption, CommandArg, RunOption};

mod dispatcher;
use dispatcher::Dispatcher;

use command::{BranchDeleteCommand, BranchFindCommand, CheckoutCommand, Command, ResetCommand,
              StatusCommand, WorkType};

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

    let is_terminal = atty::is(atty::Stream::Stdout);
    let stream = color_printer::StandardStream::stdout(color_printer::ColorChoice::Auto);
    let printer = ColorPrinter::new(is_terminal, &stream);

    let command: Box<Command> = match run_option {
        RunOption::Checkout { branch } => CheckoutCommand::box_new(branch),
        RunOption::Branch { branch, option } => match option {
            BranchOption::Delete => BranchDeleteCommand::box_new(branch),
            BranchOption::Find => BranchFindCommand::box_new(branch),
        },
        RunOption::Reset => ResetCommand::box_new(),
        RunOption::Status => StatusCommand::box_new(),
    };

    let pool = threadpool::Builder::new().build();
    let rx = start_repo_iter(&pool);

    let mut dispatcher = Dispatcher::new(&pool, printer, command);
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
