extern crate git2;
extern crate colored;

use std::error::Error;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

mod git;

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

        if let Ok(iter) = read_result {
            let path_iter = iter.filter(|x| x.is_ok())
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
                let p = entry.path();
                let changes = git::changes(p.as_path());

                if let Err(git::GitError::OpenRepo) = changes {
                    pending.push(p);
                }
            }
        }

        if pending.len() == 0 {
            break;
        }
    }

    Ok(())
}