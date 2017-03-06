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
            let non_repo_iter = read_result.unwrap()
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
                })
                .filter(|x| match git_changes(&x.path()) {  
                    Err(GitError::OpenRepo) => true, // Only return folders that arent repos
                    _ => false,
                })
                .map(|x| x.path().to_path_buf());

            for path in non_repo_iter {
                pending.push(path);
            }
        }

        if pending.len() == 0 {
            break;
        }
    }

    Ok(())
}

enum GitError {
    OpenRepo,
    Status,
}

// This can become its own iterator
fn git_changes(path: &Path) -> Result<(), GitError> {
    let repo = git2::Repository::open(path).map_err(|_| GitError::OpenRepo)?;

    if repo.is_bare() {
        return Ok(());
    }

    let mut opts = git2::StatusOptions::new();

    opts.include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_unreadable_as_untracked(true)
        .disable_pathspec_match(true)
        .exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts)).map_err(|_| GitError::Status)?;
    let mut statuses_iter = statuses.iter()
        .filter(|x| {
            if x.status() != git2::STATUS_WT_DELETED {
                return true;
            }

            // For some reason, some files with the deleted
            // status actually still exist, so ignore these.
            match x.path() {
                None => false,
                Some(p) => {
                    let mut del_path = path.to_path_buf();
                    del_path.push(p);
                    !del_path.exists()
                }
            }
        })
        .peekable();

    if statuses_iter.peek().is_none() {
        return Ok(());
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

    Ok(())
}
