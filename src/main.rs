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
                .filter(|x| match git::changes(&x.path()) {  
                    Err(git::GitError::OpenRepo) => true, // Only return folders that arent repos
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