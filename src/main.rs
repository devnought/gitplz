extern crate gitlib;
extern crate term_painter;

use std::error::Error;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

use term_painter::Color::{BrightRed, BrightCyan, BrightGreen, BrightMagenta};
use term_painter::ToStyle;

use gitlib::FileStatus;
use gitlib::{GitError, GitRepo};

fn main() {
    let working_dir = match env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            println!("Error getting working directory: {}", err.description());
            return;
        }
    };

    match walk_dirs(&working_dir) {
        Err(e) => println!("{:?}", e),
        _ => {}
    }
}

fn walk_dirs(path: &Path) -> io::Result<()> {
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
            let changes = changes(&entry.path());

            if let Err(GitError::OpenRepo) = changes {
                pending.push(entry.path().to_path_buf());
            }
        }

        if pending.len() == 0 {
            break;
        }
    }

    Ok(())
}

fn changes(path: &Path) -> Result<(), GitError> {
    let repo = GitRepo::new(path)?;
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