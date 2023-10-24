use entropy;

use lista2::lib;

fn main() {
    let file_paths: lib::helpers::Files = lib::helpers::get_args();
    println!("{}\n{}", file_paths.input_path, file_paths.output_path);
    let content: String = lib::helpers::read_file(&file_paths.input_path);
    let bytes: &[u8] = content.as_bytes();
    let entropy = entropy::entropy(&bytes);
    let byte_count: Vec<u32> = entropy::count_bytes(&bytes);
    let probability: Vec<f64> = entropy::probability(&bytes);
    let length: usize = content.len();
    println!("Długość: {length}\nEntropia: {entropy}");
}
