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

use gitlib::FileStatus;
use gitlib::{GitError, GitRepo};

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
// TODO: Try turning this into a custom iterator
fn walk_dirs(option: &mut RunOption, path: &Path) {
    let mut pending = vec![path.to_owned()];

    while let Some(current_dir) = pending.pop() {
        let read_result = current_dir.read_dir();

        if let Err(_) = read_result {
            continue;
        }

        let path_iter = read_result
            .unwrap()
            .filter_map(|x| x.ok())
            .filter(|x| x.file_type().map(|t| t.is_dir()).unwrap_or(false))
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
            let status = process(option, &entry.path());

            if let Err(GitError::OpenRepo) = status {
                pending.push(entry.path().to_path_buf());
            }
        }
    }
}

fn process(option: &mut RunOption, path: &Path) -> Result<(), GitError> {
    let repo = GitRepo::new(path)?;

    match *option {
        RunOption::Checkout(ref branch) => checkout(repo, path, branch),
        RunOption::Manifest(ref mut opt) => manifest(path, opt),
        RunOption::Reset => reset(repo, path),
        RunOption::Status => status(repo, path),
    }
}

fn checkout(repo: GitRepo, path: &Path, branch: &str) -> Result<(), GitError> {
    repo.checkout(branch)?;

    println!("{}", path.to_str().unwrap());
    //println!("    {}", BrightCyan.paint(branch));

    Ok(())
}

fn manifest(path: &Path, opt: &mut ManifestOption) -> Result<(), GitError> {
    let path_str = path.to_str().unwrap();

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

fn reset(repo: GitRepo, path: &Path) -> Result<(), GitError> {
    let head = repo.reset()?;
    let branch = BrightCyan.paint(head.name().unwrap());
    let l_brace = BrightYellow.paint("[");
    let r_brace = BrightYellow.paint("]");

    println!("{}  {}{}{}",
             path.to_str().unwrap(),
             l_brace,
             branch,
             r_brace);

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