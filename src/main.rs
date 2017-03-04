// TODO: Fix unwraps

extern crate git2;
extern crate term_painter;

use std::error::Error;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

use term_painter::{ToStyle, Color};

fn main() {
    let working_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error getting working directory: {}", err.description());
            return;
        }
    };

    println!("{:?}", walk_dirs(&working_dir));
}

fn walk_dirs(path: &Path) -> io::Result<()> {
    let mut pending: Vec<PathBuf> = Vec::new();

    loop {
        let current_dir = if pending.len() == 0 {
            path.to_path_buf()
        } else {
            pending.pop().unwrap()
        };

        let read_result = current_dir.read_dir();

        if read_result.is_ok() {
            let iter = read_result.unwrap()
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .filter(|x| match x.file_type() {
                    Ok(t) => t.is_dir(),
                    Err(_) => false,
                })
                .filter(|x| match x.path().file_name() {
                    Some(name) => name.ne(".git") && !name.to_str().unwrap().starts_with("$"),
                    None => false,
                });

            for entry in iter {
                let mut git_path = entry.path();
                git_path.push(".git");

                if git_path.exists() {
                    git_changes(&entry.path());
                } else {
                    pending.push(entry.path().to_path_buf());
                }
            }
        }

        if pending.len() == 0 {
            break;
        }
    }

    Ok(())
}

fn git_changes(path: &Path) {
    let repo = git2::Repository::open(path).unwrap();

    if repo.is_bare() {
        return;
    }

    let mut opts = git2::StatusOptions::new();

    opts.include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_unreadable_as_untracked(true)
        .disable_pathspec_match(true)
        .exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts)).unwrap();
    let mut statuses_iter = statuses.iter()
        .filter(|x| {
            if x.status() != git2::STATUS_WT_DELETED {
                return true;
            }

            // For some reason, some files with the deleted
            // status actually still exist, so ignore these.
            let mut del_path = path.to_path_buf();
            del_path.push(x.path().unwrap());

            !del_path.exists()
        })
        .peekable();

    if statuses_iter.peek().is_none() {
        return;
    }

    println!("{}", path.to_str().unwrap());

    for entry in statuses_iter {
        let pre = match entry.status() {
            git2::STATUS_WT_DELETED => Color::BrightRed.paint("    Deleted"),
            git2::STATUS_WT_MODIFIED => Color::BrightCyan.paint("   Modified"),
            git2::STATUS_WT_NEW => Color::BrightGreen.paint("        New"),
            git2::STATUS_WT_RENAMED => Color::BrightCyan.paint("    Renamed"),
            git2::STATUS_WT_TYPECHANGE => Color::BrightCyan.paint("Typechanged"),
            _ => Color::BrightMagenta.paint("    Unknown"),
        };

        println!("  {} {}", pre, entry.path().unwrap());
    }
}
