#[derive(Debug)]
pub struct Dane {
    pub file_path: String,
    pub table: bool,
}

pub fn read_args() -> Dane {
    let mut args = std::env::args();
    let mut file_path: Option<String> = None;
    let mut table = false;
    while let Some(s) = args.next() {
        if s == "--table" {
            table = true;
        } else if s == "--src" {
            file_path = args.next();
        }
    }

    match file_path {
        Some(s) => Dane {
            file_path: s,
            table,
        },
        None => {
            eprintln!("Error: the file must be specified with [--src] flag");
            std::process::exit(1);
        }
    }
}

pub fn read_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    })
}
