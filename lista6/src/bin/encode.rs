use std::env;
use std::fs::File;
use std::io::Write;

use lista6::*;
use log::info;

fn main() {
	simple_logger::SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Wrong argument count\nUsage: encode <input_file> <output_file> <bitoffset>")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let bitoffset: u8 = args.get(3).unwrap().parse().unwrap();

    if bitoffset < 1 || bitoffset > 8 {
        panic!("Bitoffset must be between 1 and 8")
    }

    let image = image::Image::from_tga_file(file_path);
    image.save_as_png(&String::from(file_path).replace(".tga", ".png"));

    let img = encode_from_tga(image, bitoffset);

    let mut output_file = File::create(output_file_path).expect("Unable to create file");

	info!("Writing to {}...", output_file_path);
    output_file
        .write_all(&img.to_bytes())
        .expect("Unable to write data");
}
