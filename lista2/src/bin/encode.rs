use std::fs::{File, read};
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
    let input = File::open(input_file)?;
    let content = read(input_file).unwrap();
    let len_before = content.len();
    let output = File::create(output_file)?;

    let cmodel = ModelA::new();
    let (len_after_bytes, len_after_bits): (usize, usize) = compress(input, output, cmodel)?;

    println!("Entropia: {}", entropy::entropy(&content));
    println!("Średnia długość kodowania: {}", len_after_bits as f64 / len_before as f64);
    println!("Stopień kompresji: {}", len_before as f64 / len_after_bytes as f64);
    Ok(())
}
