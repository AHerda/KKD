#[derive(Debug, PartialEq)]
enum PathType {
    File,
    Dir,
    None,
}

#[derive(Debug)]
pub struct Dane {
    pub path_type: PathType,
    pub path: String,
    pub table: bool,
}

pub fn read_args() -> Dane {
    let mut args: std::env::Args = std::env::args();

    let mut path_type: PathType = PathType::None;
    let mut path: Option<String> = None;
    let mut table: bool = false;

    while let Some(s) = args.next() {
        if s == "--table" {
            table = true;
        } else if s == "--src" {
            path = args.next();
            if path_type == PathType::None {
                path_type = PathType::File;
            } else {
                eprintln!("Error: You can onlu specify ONE path");
                std::process::exit(1);
            }
        } else if s == "--src-dir" {
            path = args.next();
            if path_type == PathType::None {
                path_type = PathType::Dir;
            } else {
                eprintln!("Error: You can onlu specify ONE path");
                std::process::exit(1);
            }
        }
    }

    let path: String = path.unwrap_or_else(|| {
        eprintln!("Error: the path must be specified after [--src] or [--src-dir] flag");
        std::process::exit(1);
    });

    match path_type {
        PathType::None => {
            eprintln!("Error: the path must be specified with [--src] or [--src-dir] flag");
            std::process::exit(1);
        },
        _ => Dane {
            path_type,
            path,
            table,
        },
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

pub fn read_dir(dir_path: &str) -> Vec<u8> {
    match std::fs::read_dir(dir_path) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}