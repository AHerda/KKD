// use entropy;

// use lista2::libs::{code::{encode, to_string}, helpers::{Files, get_args, read_file}};

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

//     let (compressed, z, m, size) = encode(bytes);
//     println!("Compressed: {}", to_string(&compressed));
//     println!("Compressed size: {size}");
//     println!("Size: {m}");
//     println!("z = {z}");
// }


use std::fs::File;
use lista2::libs::{model_a::ModelA, compressor::compress};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("missing command line arguments");
        std::process::exit(255);
    }
    match run(&args[1], &args[2]) {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("Failed with exception: {}", err);
            std::process::exit(255);
        }
    }
}

fn run(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    let mut cmodel = ModelA::new();
    cmodel.model_metrics.dump("cmodel");

    println!("compressing...");
    compress(input, output, cmodel)?;

    // println!("{}", cmodel.m_bytes_processed);
    Ok(())
}