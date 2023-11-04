// use entropy;

// use lista2::libs::{code::{decode, to_string}, helpers::{Files, get_args, read_file}};

// fn main() {
//     let file_paths: Files = get_args();
//     println!("{}\n{}", file_paths.input_path, file_paths.output_path);
//     let content: String = read_file(&file_paths.input_path);
//     let bytes: &[u8] = content.as_bytes();
//     let entropy = entropy::entropy(&bytes);
//     let _byte_count: Vec<u32> = entropy::count_bytes(&bytes);
//     let _probability: Vec<f64> = entropy::probability(&bytes);
//     let length: usize = content.len();
//     println!("Długość: {length}\nEntropia: {entropy}");

//     let (compressed, size) = decode(bytes);
//     println!("Compressed: {}", to_string(&compressed));
//     println!("Size: {size}");
//     println!("z = {z}");
// }

use std::fs::File;
use lista2::libs::decompressor::decompress;
use lista2::libs::model_a::ModelA;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("missing command line arguments");
        std::process::exit(255);
    }
    match File::open(&args[1]) {
        Ok(input) => {
            match File::create(&args[2]) {
                Ok(output) => {
                    let cmodel: ModelA = ModelA::new();
                    cmodel.model_metrics.dump("cmodel");
                    println!("decompressing...");
                    _ = decompress(input, output, cmodel);
                    // println!("{}", cmodel.bytes_processed);
                    std::process::exit(0);
                }
                Err(err) => eprintln!("Failed to create output file: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to open input file: {}", err),
    }
    std::process::exit(255);
}


