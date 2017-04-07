#[macro_use]
extern crate clap;

extern crate gitlib;
extern crate term_painter;
extern crate pbr;

use std::error::Error;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta};
use term_painter::ToStyle;

use gitlib::FileStatus;
use gitlib::{GitError, GitRepo};

mod cli;

#[derive(Debug)]
enum RunOptions {
    Checkout(String),
    Reset,
    Status,
}

fn main() {
    let working_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error getting working directory: {}", err.description());
            return;
        }
    };

    let matches = cli::build_cli().get_matches();

    let option = match matches.subcommand_name() {
        Some(cli::CMD_CHECKOUT) => {
            let branch_match = matches.subcommand_matches(cli::CMD_CHECKOUT).unwrap();
            let branch = value_t!(branch_match, cli::BRANCH, String).unwrap();
            RunOptions::Checkout(branch)
        }
        Some(cli::CMD_COMPLETIONS) => {
            if let Some(ref matches) = matches.subcommand_matches(cli::CMD_COMPLETIONS) {
                let shell = value_t!(matches, cli::SHELL, clap::Shell).unwrap();
                cli::build_cli().gen_completions_to(cli::APP_NAME, shell, &mut std::io::stdout());
            }

            return;
        }
        Some(cli::CMD_RESET) => RunOptions::Reset,

        // By default, just show status.
        _ => RunOptions::Status,
    };

    match walk_dirs(&option, &working_dir) {
        Err(e) => println!("{:?}", e),
        _ => {}
    }
}

fn walk_dirs(options: &RunOptions, path: &Path) -> io::Result<()> {
    let mut pending: Vec<PathBuf> = Vec::new();

    loop {
        let current_dir = pending.pop().unwrap_or(path.to_owned());
        let read_result = current_dir.read_dir();

        if let Err(_) = read_result {
            if pending.len() == 0 {
                break;
            }

            continue;
        }

        let path_iter = read_result
            .unwrap()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .filter(|x| match x.file_type() {
                        Ok(t) => t.is_dir(),
                        Err(_) => false,
                    })
            .filter(|x| match x.path().file_name() {
                        Some(name) => {
                            match name.to_str() {
                                Some(name_str) => {
                                    !name_str.starts_with(".") && !name_str.starts_with("$")
                                }
                                None => false,
                            }
                        }
                        None => false,
                    });

        for entry in path_iter {
            let status = process(options, &entry.path());

            if let Err(GitError::OpenRepo) = status {
                pending.push(entry.path().to_path_buf());
            }
        }

        if pending.len() == 0 {
            break;
        }
    }

    Ok(())
}

fn process(options: &RunOptions, path: &Path) -> Result<(), GitError> {
    let repo = GitRepo::new(path)?;

    match *options {
        RunOptions::Checkout(ref branch) => checkout(repo, path, branch),
        RunOptions::Reset => reset(repo, path),
        RunOptions::Status => status(repo, path),
    }
}

fn checkout(repo: GitRepo, path: &Path, branch: &str) -> Result<(), GitError> {
    Ok(())
}

fn reset(repo: GitRepo, path: &Path) -> Result<(), GitError> {
    let head = repo.reset()?;

    println!("{}", path.to_str().unwrap());
    println!("  {}", head.name().unwrap());

    Ok(())
}

fn status(repo: GitRepo, path: &Path) -> Result<(), GitError> {
    let statuses = repo.statuses()?;

    if statuses.len() == 0 {
        return Ok(());
    }

    println!("{}", path.to_str().unwrap());

    for entry in statuses.iter() {
        let (pre, colour) = match entry.status() {
            FileStatus::Deleted => ("    Deleted", BrightRed),
            FileStatus::Modified => ("   Modified", BrightCyan),
            FileStatus::New => ("        New", BrightGreen),
            FileStatus::Renamed => ("    Renamed", BrightCyan),
            FileStatus::Typechanged => ("Typechanged", BrightCyan),
            FileStatus::Unknown => ("    Unknown", BrightMagenta),
        };

        println!("  {} {}", colour.paint(pre), entry.path().unwrap());
    }

    Ok(())
}