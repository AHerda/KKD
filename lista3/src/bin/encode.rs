use entropy::entropy;
use std::env;
use std::fs::File;
use std::io::Write;

use lista32::libs::{
    coding_types::CodingType::{DELTA, FIB, GAMMA, OMEGA},
    encoder::Encoder,
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

    println!("encodee size: {:?} B", file.len());
    println!("encodee entropy: {:?}", entropy(&file));

    let encoder = Encoder::new(encoding_type);
    let encoded = encoder.encode(&file, true);
    let encoded_bytes = encoded.to_bytes();
    println!("encoding size: {:?} B", encoded_bytes.len());
    println!("encoding entropy: {:?}", entropy(&encoded_bytes));
    println!(
        "compression ratio: {:?}",
        file.len() as f64 / encoded_bytes.len() as f64
    );

    let mut output = File::create(output_file_path).unwrap();
    output.write_all(&encoded_bytes).unwrap();
}
