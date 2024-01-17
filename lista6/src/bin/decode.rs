use std::env;
use std::fs::File;
use std::io::Write;

use lista6::*;
use log::info;

fn main() {
	simple_logger::SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();

    let img = decode_from_bin(file_path);

    image::Image::from_tga(&img).save_as_png(&output_file_path.replace(".tga", ".png"));

    let mut output_file = File::create(output_file_path).expect("Unable to create file");

	info!("Writing to {}...", output_file_path);
    output_file
		.write_all(&img)
		.expect("Unable to write data");
}
