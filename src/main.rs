#[macro_use]
extern crate clap;
extern crate app_dirs;
extern crate term_painter;

extern crate gitlib;
extern crate util;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta, BrightYellow};
use term_painter::ToStyle;

use gitlib::{FileStatus, GitError, GitRepo};
use util::{GitRepositories, Manifest};

use app_dirs::{AppInfo, AppDataType};

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
    Clean,
    Preview,
    Update,
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
        Some(cli::CMD_MANIFEST) => {
            let matches = matches.subcommand_matches(cli::CMD_MANIFEST).unwrap();

            match matches.subcommand_name() {
                Some(cli::CMD_CLEAN) => RunOption::Manifest(ManifestOption::Clean),
                Some(cli::CMD_UPDATE) => RunOption::Manifest(ManifestOption::Update),
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

    process(&option, &working_dir);
}

fn process(option: &RunOption, path: &Path) {
    let manifest_path = build_manifest_path();
    let mut manifest = Manifest::open(&manifest_path, &path);

    if let RunOption::Manifest(ref m) = *option {
        let repos = GitRepositories::new(path);

        match *m {
            ManifestOption::Clean => manifest_clean(&manifest_path),
            ManifestOption::Preview => manifest_preview(repos),
            ManifestOption::Update => manifest_update(repos, &mut manifest),
        }
        return;
    }

    let repos = match manifest.is_empty() {
        true => GitRepositories::new(path),
        false => GitRepositories::from_manifest(&manifest),
    };

    for repo in repos {
        match *option {
            RunOption::Checkout(ref branch) => {
                checkout(&repo, branch).unwrap_or_else(|_| println!("Error on checkout"))
            }
            RunOption::Reset => reset(&repo).unwrap(),
            RunOption::Status => status(&repo).unwrap(),
            _ => panic!("Unhandled run option"),
        }
    }
}

fn build_manifest_path() -> PathBuf {
    const APP_INFO: AppInfo = AppInfo {
        name: "git-plz",
        author: "devnought",
    };

    let root = app_dirs::get_app_root(AppDataType::UserCache, &APP_INFO).expect("Could not locate app settings directory");
    let mut path = PathBuf::from(root);
    path.push("manifest.json");

    path
}

fn checkout(repo: &GitRepo, branch: &str) -> Result<(), GitError> {
    repo.checkout(branch)?;

    println!("{}", repo.path().to_str().unwrap());
    //println!("    {}", BrightCyan.paint(branch));

    Ok(())
}

fn manifest_update(repos: GitRepositories, manifest: &mut Manifest) {
    manifest.add_repositories(repos);

    println!("{:#?}", &manifest);
}

fn manifest_preview(repos: GitRepositories) {
    for repo in repos {
        println!("{}", repo.path().to_str().unwrap());
    }
}

fn manifest_clean(manifest_path: &Path) {
    println!("Attempting to delete: {:?}", manifest_path);

    if manifest_path.exists() {
        fs::remove_file(manifest_path).unwrap()
    }
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