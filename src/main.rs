extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate num_cpus;
extern crate term_painter;
extern crate threadpool;

extern crate gitlib;
extern crate util;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;

use app_dirs::{AppInfo, AppDataType};
use indicatif::{ProgressBar, ProgressStyle};
use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta, BrightYellow};
use term_painter::ToStyle;
use threadpool::ThreadPool;

use gitlib::{FileStatus, GitError, GitRepo};
use util::{GitRepositories, Manifest};

mod cli;

#[derive(Debug, Clone)]
enum RunOption {
    Checkout(String),
    Manifest(ManifestOption),
    Reset,
    Status,
}

#[derive(Debug, Clone)]
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

    process(option, &working_dir);
}

fn process(option: RunOption, path: &Path) {
    let manifest_path = build_manifest_path();
    let mut manifest = Manifest::open(&manifest_path, &path);

    if let RunOption::Manifest(ref m) = option {
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

    let thread_count = num_cpus::get();
    let pool = ThreadPool::new(thread_count);
    let (tx, rx) = channel();
    let mut repo_count = 0;

    let pb = ProgressBar::new_spinner();

    for repo in repos {
        let tx = tx.clone();

        pool.execute(move || {
            let statuses = match repo.statuses() {
                Ok(s) => s,
                Err(_) => {
                    tx.send(None).expect("Could not signal main thread");
                    return;
                }
            };

            if statuses.len() == 0 {
                tx.send(None).expect("Could not signal main thread");
                return;
            }

            let statuses_result = statuses.iter().collect::<Vec<_>>();

            tx.send(Some((repo.path().to_path_buf(), statuses_result)))
                .expect("Could not signal main thread");
        });

        repo_count += 1;
    }

    let mut completed = 0;

    for tuple in rx.iter().take(repo_count) {
        completed += 1;

        if let None = tuple {
            continue;
        }

        let (path, list) = tuple.unwrap();

        println!("{}", path.to_str().unwrap());

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

            println!("  {} {}",
                     colour.paint(pre),
                     entry.path().expect("Could not unwrap entry path"));
        }
    }

    assert_eq!(completed as usize, repo_count);
}

fn build_manifest_path() -> PathBuf {
    const APP_INFO: AppInfo = AppInfo {
        name: "git-plz",
        author: "devnought",
    };

    let root = app_dirs::get_app_root(AppDataType::UserCache, &APP_INFO)
        .expect("Could not locate app settings directory");
    let mut path = PathBuf::from(root);
    path.push("manifest.json");

    path
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

fn checkout(repo: &GitRepo, branch: &str) -> Result<(), GitError> {
    repo.checkout(branch)?;

    println!("{}",
             repo.path()
                 .to_str()
                 .expect("Could not unwrap repo path"));
    println!("    {}", BrightCyan.paint(branch));

    Ok(())
}

fn reset(repo: &GitRepo) -> Result<(), GitError> {
    // This might not actually give any performance boost
    if repo.statuses()?.len() == 0 {
        return Ok(());
    }

    repo.remove_untracked()?;

    let head = repo.reset()?;
    let branch = BrightCyan.paint(head.name().expect("Error unwrapping head name"));
    let l_brace = BrightYellow.paint("[");
    let r_brace = BrightYellow.paint("]");

    println!("  {}{}{}  {}",
             l_brace,
             branch,
             r_brace,
             repo.path()
                 .to_str()
                 .expect("Error unwrapping repo path"));

    Ok(())
}

/*
fn status(repo: &GitRepo) -> Result<(), GitError> {
    let statuses = repo.statuses()?;

    if statuses.len() == 0 {
        return Ok(());
    }

    println!("{}",
             repo.path()
                 .to_str()
                 .expect("Could not unwrap repo path"));

    for entry in statuses.iter() {
        let (pre, colour) = match entry.status() {
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

        println!("  {} {}",
                 colour.paint(pre),
                 entry.path().expect("Could not unwrap entry path"));
    }

    Ok(())
}
*/
