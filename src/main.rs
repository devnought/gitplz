#[macro_use]
extern crate clap;
extern crate term_painter;
extern crate threadpool;

extern crate gitlib;
extern crate util;

use std::env;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

use term_painter::Color::{BrightCyan, BrightWhite, BrightYellow};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use util::GitRepositories;

mod cli;
mod common;
mod status;
mod checkout;

const THREAD_SIGNAL: &str = "Could not signal main thread";

#[derive(Debug, Clone)]
enum RunOption {
    Checkout(String),
    Reset,
    Status,
}

fn main() {
    let working_dir = env::current_dir().expect("Could not get working directory");
    let matches = cli::build_cli().get_matches();

    let option = match matches.subcommand_name() {
        Some(cli::CMD_CHECKOUT) => {
            let branch_match = matches.subcommand_matches(cli::CMD_CHECKOUT).unwrap();
            let branch = value_t!(branch_match, cli::BRANCH, String).unwrap();
            RunOption::Checkout(branch)
        }
        Some(cli::CMD_COMPLETIONS) => {
            if let Some(matches) = matches.subcommand_matches(cli::CMD_COMPLETIONS) {
                let shell = value_t!(matches, cli::SHELL, clap::Shell).unwrap();
                cli::build_cli().gen_completions_to(cli::APP_NAME, shell, &mut std::io::stdout());
            }

            return;
        }
        Some(cli::CMD_RESET) => RunOption::Reset,
        Some(cli::CMD_STATUS) => RunOption::Status,

        // By default, show help.
        _ => {
            cli::build_cli()
                .print_help()
                .expect("Could not print command line help message");
            return;
        }
    };

    process(option, &working_dir);
}

fn process(option: RunOption, path: &Path) {
    let repos = GitRepositories::new(path);
    let pool = threadpool::Builder::new().build();

    match option {
        RunOption::Reset => {
            let rx = reset(repos, &pool);

            while let Ok((path, head)) = rx.recv() {
                let branch = BrightCyan.paint(head);
                let l_brace = BrightYellow.paint("[");
                let r_brace = BrightYellow.paint("]");

                println!("  {}{}{}  {}", l_brace, branch, r_brace, path.display());
            }
        }
        RunOption::Status => status::process_status(repos, &pool),
        RunOption::Checkout(branch) => match checkout::process_checkout(repos, &branch, &pool) {
            Ok(branches) => {
                let ess = match branches {
                    1 => "",
                    _ => "s",
                };

                println!(
                    "Checkout finished, checked out branch on {} repo{}",
                    branches, ess
                );
            }
            Err(msg) => println!("Checkout blew up, no checkout for you: {:#?}", msg),
        },
    }
}

fn reset(repos: GitRepositories, pool: &ThreadPool) -> Receiver<(PathBuf, String)> {
    let (tx, rx) = channel();

    for repo in repos {
        let tx = tx.clone();

        pool.execute(move || {
            if let Ok(s) = repo.statuses() {
                if s.is_empty() {
                    return;
                }
            }

            if repo.remove_untracked().is_err() {
                return;
            }

            let head = match repo.reset() {
                Ok(r) => r,
                _ => return,
            };

            let tuple = (repo.path().to_path_buf(), head.name().to_string());
            tx.send(tuple).expect(THREAD_SIGNAL);
        });
    }

    rx
}
