use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::{fs, io};

#[derive(Debug, PartialEq)]
pub enum PathType {
    File(String),
    Dir(String),
    None,
}

#[derive(Debug)]
pub struct Dane {
    pub path: PathType,
    pub table: bool,
}

pub fn read_args() -> Dane {
    let mut args: std::env::Args = std::env::args();

    let mut path: PathType = PathType::None;
    let mut table: bool = false;

    while let Some(s) = args.next() {
        if s == "--table" {
            table = true;
        } else if s == "--src" {
            if path == PathType::None {
                path = PathType::File(args.next().unwrap_or_else(|| {
                    eprintln!(
                        "Error: the path must be specified after [--src] or [--src-dir] flag"
                    );
                    std::process::exit(1);
                }));
            } else {
                eprintln!("Error: You can only specify ONE path");
                std::process::exit(1);
            }
        } else if s == "--src-dir" {
            if path == PathType::None {
                path = PathType::Dir(args.next().unwrap_or_else(|| {
                    eprintln!(
                        "Error: the path must be specified after [--src] or [--src-dir] flag"
                    );
                    std::process::exit(1);
                }));
            } else {
                eprintln!("Error: You can onlu specify ONE path");
                std::process::exit(1);
            }
        }
    }

    match path {
        PathType::None => {
            eprintln!("Error: the path must be specified with [--src] or [--src-dir] flag");
            std::process::exit(1);
        }
        _ => Dane { path, table },
    }
}

pub fn read_file(file_path: &str) -> Vec<u8> {
    match std::fs::read(file_path) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

pub fn format_srcs(dir_path: &str) -> Vec<(String, Vec<u8>)> {
    match read_dir_mine(dir_path) {
        Ok(v) => v
            .par_iter()
            .map(|file_path| (file_path.clone(), read_file(file_path)))
            .collect(),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

fn read_dir_mine(dir_path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_dir(dir_path)?
        .map(|file_path| file_path.unwrap().path().to_str().unwrap().to_string())
        .collect())
}
