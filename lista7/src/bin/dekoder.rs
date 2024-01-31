use std::env;
use std::fs::File;
use std::io::Write;

use lista7::bitvec::BitVec;
use lista7::hamming_decoding;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }

    let input_file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let input_file = BitVec::from_bytes(std::fs::read(input_file_path).unwrap());

    let mut output_file = File::create(output_file_path).unwrap();
    let mut packet = Vec::new();
    let mut decoding = BitVec::new();
    let mut uncorrectable = 0;

    for bit in input_file {
        packet.push(bit);
        if packet.len() == 8 {
            let res = match hamming_decoding(packet.clone()) {
                Ok(bitvec) => bitvec,
                Err(option) => {
                    uncorrectable += 1;
                    option.unwrap()
                }
            };

            for bit in res {
                decoding.push(bit);
            }

            packet.clear();
        }
    }

    println!("\x1b[91mUncorrectable Errors: {}\x1b[0m", uncorrectable);
    output_file.write_all(&decoding.to_bytes()).unwrap();
}
