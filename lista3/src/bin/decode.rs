use std::env;
use std::fs::File;
use std::io::Write;

use lista32::libs::{
    bitvec::BitVec,
    coding_types::CodingType::{DELTA, FIB, GAMMA, OMEGA},
    decoder::Decoder,
};

fn main() {
    // Take in and vaidate input
    let args: Vec<String> = env::args().collect();

    assert!(args.len() >= 3, "Wrong argument count");

    let encoding_type = match args.get(3) {
        Some(enc_str) => match enc_str.as_str() {
            "gamma" => GAMMA,
            "delta" => DELTA,
            "fib" => FIB,
            _ => OMEGA,
        },
        None => OMEGA,
    };
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let file = std::fs::read(file_path).unwrap();

    let decoder = Decoder::new(encoding_type);
    let decoded = decoder.decode(BitVec::from_bytes(&file));

    let mut output = File::create(output_file_path).unwrap();
    output.write_all(&decoded).unwrap();
}
