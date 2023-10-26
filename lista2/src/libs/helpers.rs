use std::{env::args, fs, process};

#[derive(PartialEq, Eq)]
pub enum Coding {
    Encode,
    Decode,
    None,
}

pub struct Files {
    pub input_path: String,
    pub output_path: String,
    pub coding: Coding,
}

pub fn get_args() -> Files {
    let mut args = args();
    if args.len() < 3 {
        eprintln!("Wrong arguments!\n./encode --[encode/decode] --input <input file path> [--output <output file path>]\n./encode -i <input file path> [-o <output file path>]");
        process::exit(-1);
    }
    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut coding: Coding = Coding::None;

    while let Some(arg) = args.next() {
        if arg == "-i" || arg == "--input" {
            input = args.next();
        } else if arg == "-o" || arg == "--output" {
            output = args.next();
        } else if arg == "--encode" {
            if coding == Coding::None {
                panic!("You can't use encoding and decoding in one command");
            }
            coding = Coding::Encode;
        } else if arg == "--decode" {
            if coding == Coding::None {
                panic!("You can't use encoding and decoding in one command");
            }
            coding = Coding::Decode;
        }
    }
    Files {
        input_path: input.expect("You Have to include input file path after"),
        output_path: output.unwrap_or("output.bin".to_string()),
        coding,
    }
}

pub fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => panic!("Wrong input-file path\n{err}"),
    }
}
