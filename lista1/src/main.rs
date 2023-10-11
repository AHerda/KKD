mod libs;

use libs::{helpers, functions};

fn main() {
    let dane: helpers::Dane = helpers::read_args();
    let file_content: String = helpers::read_file(dane.file_path);
    let total_bytes = file_content.len();
    let bytes_counted: Vec<u32> = functions::count_bytes(file_content);

    // println!("{:?}", bytes_counted);
    // bytes_counted.iter().enumerate().for_each(|(byte, value)| println!("{}: {}", byte as u8 as char, value));

    let probability = functions::probability(bytes_counted, total_bytes as u32);

    // println!("{:?}", probability);
    // probability.iter().enumerate().for_each(|(byte, value)| println!("{}: {}", byte as u8 as char, value));
}