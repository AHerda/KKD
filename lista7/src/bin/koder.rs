use std::env;
use std::fs::File;
use std::io::Write;

use lista7::bitvec::BitVec;
use lista7::hamming_encoding;

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
    let mut encodeing = BitVec::new();

    for bit in input_file {
        packet.push(bit);
        if packet.len() == 4 {
            let res = hamming_encoding(packet.clone()).unwrap();

            for bit in res {
                encodeing.push(bit);
            }

            packet.clear();
        }
    }
    output_file.write_all(&encodeing.to_bytes()).unwrap();
}
