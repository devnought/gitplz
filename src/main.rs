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

const THREAD_SIGNAL: &str = "Could not signal main thread";

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


    let pb = ProgressBar::new_spinner();

    match option {
        RunOption::Reset => reset(repos, pool),
        RunOption::Status => status(repos, pool),
        _ => panic!("Unhandled run option"),
    }
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

fn reset(repos: GitRepositories, pool: ThreadPool) {
    let (tx, rx) = channel();
    let mut repo_count = 0;

    for repo in repos {
        let tx = tx.clone();
        repo_count += 1;

        pool.execute(move || {
            match repo.statuses() {
                Ok(s) => {
                    if s.len() == 0 {
                        tx.send(None).expect(THREAD_SIGNAL);
                        return;
                    }
                }
                _ => (),
            }

            match repo.remove_untracked() {
                Err(_) => {
                    tx.send(None).expect(THREAD_SIGNAL);
                    return;
                }
                _ => (),
            }

            let head = match repo.reset() {
                Ok(r) => r,
                _ => {
                    tx.send(None).expect(THREAD_SIGNAL);
                    return;
                }
            };

            let tuple = (repo.path().to_path_buf(), head.name().to_string());
            tx.send(Some(tuple)).expect(THREAD_SIGNAL);
        });
    }

    let mut completed = 0;

    while repo_count > completed {
        completed += 1;

        let tuple = match rx.recv() {
            Ok(h) => h,
            Err(_) => break,
        };

        if let None = tuple {
            continue;
        }

        let (path, head) = tuple.unwrap();

        let branch = BrightCyan.paint(head);
        let l_brace = BrightYellow.paint("[");
        let r_brace = BrightYellow.paint("]");

        println!("  {}{}{}  {}", l_brace, branch, r_brace, path.display());
    }

    assert_eq!(completed, repo_count);
}

fn status(repos: GitRepositories, pool: ThreadPool) {
    let (tx, rx) = channel();
    let mut repo_count = 0;

    for repo in repos {
        let tx = tx.clone();
        repo_count += 1;

        pool.execute(move || {
            let statuses = match repo.statuses() {
                Ok(s) => s,
                Err(_) => {
                    tx.send(None).expect(THREAD_SIGNAL);
                    return;
                }
            };

            if statuses.len() == 0 {
                tx.send(None).expect(THREAD_SIGNAL);
                return;
            }

            let statuses_result = statuses.iter().collect::<Vec<_>>();
            let tuple = (repo.path().to_path_buf(), statuses_result);
            tx.send(Some(tuple)).expect(THREAD_SIGNAL);
        });
    }

    let mut completed = 0;

    while repo_count > completed {
        completed += 1;

        let tuple = match rx.recv() {
            Ok(t) => t,
            Err(_) => break,
        };

        if let None = tuple {
            continue;
        }

        let (path, list) = tuple.unwrap();

        println!("{}", path.display());

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

            println!("  {} {}", colour.paint(pre), entry.path().display());
        }
    }

    assert_eq!(completed, repo_count);
}
