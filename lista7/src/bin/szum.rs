use std::env;
use std::fs::File;
use std::io::Write;

use rand::prelude::*;
use rand_pcg::Pcg64;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Wrong argument count")
    }

    let p: f64 = args.get(1).unwrap().parse().unwrap();
    let input_file_path = args.get(2).unwrap();
    let output_file_path = args.get(3).unwrap();

    let mut file = std::fs::read(input_file_path).unwrap();
    let mut rng = Pcg64::from_entropy();

    for (i, byte) in file.clone().iter().enumerate() {
        let mut tmp_byte = byte.clone();

        for n in 0..8 {
            if rng.gen_bool(p) {
                tmp_byte ^= 1 << n;
            }
        }

        file[i] = tmp_byte;
    }

    let mut output_file = File::create(output_file_path).expect("Unable to create file");
    output_file.write_all(&file).expect("Unable to write data");
}
