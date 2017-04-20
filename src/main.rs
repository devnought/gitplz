#[macro_use]
extern crate clap;

extern crate gitlib;
extern crate term_painter;
extern crate pbr;

use std::error::Error;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta, BrightYellow};
use term_painter::ToStyle;

use gitlib::{FileStatus, GitError, GitRepo, GitRepositories};

mod cli;

#[derive(Debug)]
enum RunOption {
    Checkout(String),
    Manifest(ManifestOption),
    Reset,
    Status,
}

#[derive(Debug)]
enum ManifestOption {
    Generate(File),
    Preview,
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

    let mut option = match matches.subcommand_name() {
        Some(cli::CMD_CHECKOUT) => {
            let branch_match = matches.subcommand_matches(cli::CMD_CHECKOUT).unwrap();
            let branch = value_t!(branch_match, cli::BRANCH, String).unwrap();
            RunOption::Checkout(branch)
        }
        Some(cli::CMD_MANIFEST) => {
            let matches = matches.subcommand_matches(cli::CMD_MANIFEST).unwrap();

            match matches.subcommand_name() {
                Some(cli::CMD_GENERATE) => {
                    let file = File::create("/manifest.txt").unwrap();
                    RunOption::Manifest(ManifestOption::Generate(file))
                }
                _ => RunOption::Manifest(ManifestOption::Preview),
            }
        }
        Some(cli::CMD_COMPLETIONS) => {
            if let Some(ref matches) = matches.subcommand_matches(cli::CMD_COMPLETIONS) {
                let shell = value_t!(matches, cli::SHELL, clap::Shell).unwrap();
                cli::build_cli().gen_completions_to(cli::APP_NAME, shell, &mut std::io::stdout());
            }

            return;
        }
        Some(cli::CMD_RESET) => RunOption::Reset,

        // By default, just show status.
        _ => RunOption::Status,
    };

    walk_dirs(&mut option, &working_dir);
}

fn walk_dirs(option: &mut RunOption, path: &Path) {
    let repos = GitRepositories::new(path);

    for repo in repos {
        match process(option, &repo) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

fn process(option: &mut RunOption, repo: &GitRepo) -> Result<(), GitError> {
    match *option {
        RunOption::Checkout(ref branch) => checkout(repo, branch),
        RunOption::Manifest(ref mut opt) => manifest(repo, opt),
        RunOption::Reset => reset(repo),
        RunOption::Status => status(repo),
    }
}

fn checkout(repo: &GitRepo, branch: &str) -> Result<(), GitError> {
    repo.checkout(branch)?;

    println!("{}", repo.path().to_str().unwrap());
    //println!("    {}", BrightCyan.paint(branch));

    Ok(())
}

fn manifest(repo: &GitRepo, opt: &mut ManifestOption) -> Result<(), GitError> {
    let path_str = repo.path().to_str().unwrap();

    match *opt {
        ManifestOption::Preview => println!("{}", path_str),
        ManifestOption::Generate(ref mut file) => {
            writeln!(file, "{}", path_str)
                .map_err(|_| GitError::Manifest)
                .unwrap()
        }
    }

    Ok(())
}

fn reset(repo: &GitRepo) -> Result<(), GitError> {
    let head = repo.reset()?;
    let branch = BrightCyan.paint(head.name().unwrap());
    let l_brace = BrightYellow.paint("[");
    let r_brace = BrightYellow.paint("]");

    println!("{}  {}{}{}",
             repo.path().to_str().unwrap(),
             l_brace,
             branch,
             r_brace);

    Ok(())
}

fn status(repo: &GitRepo) -> Result<(), GitError> {
    let statuses = repo.statuses()?;

    if statuses.len() == 0 {
        return Ok(());
    }

    println!("{}", repo.path().to_str().unwrap());

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