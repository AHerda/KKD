use std::env;
use std::fs::File;
use std::io::{self, Read};

use log::info;

use lista7::{color_not_matching, string_slice_from_buffer};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }

    let file0_path = &args[1];
    let file1_path = &args[2];

    let mut file0 = File::open(file0_path)?;
    let mut file1 = File::open(file1_path)?;

    let mut buf0 = Vec::new();
    let mut buf1 = Vec::new();

    file0.read_to_end(&mut buf0)?;
    file1.read_to_end(&mut buf1)?;

    let mut errors = 0;

    for i in 0..buf0.len() {
        let byte0 = buf0[i];
        let byte1 = buf1[i];

        let segment0 = byte0 >> 4;
        let segment1 = byte1 >> 4;

if segment0 != segment1 {
	let (s1, s2) = color_not_matching(string_slice_from_buffer(&buf0, i, 100), string_slice_from_buffer(&buf1, i, 100));
	info!(
		"Found error in the left part of byte {}\n\t{} != {}\n\t{}\n\t{}",
		i,
		byte0 as char,
		byte1 as char,
		s1,
		s2,
	);
	errors += 1;
}

let segment0 = byte0 << 4;
        let segment1 = byte1 << 4;

        if segment0 != segment1 {
			let (s1, s2) = color_not_matching(string_slice_from_buffer(&buf0, i, 100), string_slice_from_buffer(&buf1, i, 100));
            info!(
                "Found error in the right part of byte {}\n\t{} != {}\n\t{}\n\t{}",
                i,
                byte0 as char,
                byte1 as char,
                s1,
                s2,
            );
            errors += 1;
        }
    }

    println!("\x1b[91mFound {} Errors\x1b[0m", errors);

    Ok(())
}
