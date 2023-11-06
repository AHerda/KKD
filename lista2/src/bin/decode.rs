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
                    //cmodel.model_metrics.dump("cmodel");
                    // println!("decompressing...");
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
