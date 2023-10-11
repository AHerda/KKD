mod libs;

use comfy_table::*;
use libs::{functions, helpers};

fn main() {
    let dane: helpers::Dane = helpers::read_args();
    let file_content: Vec<u8> = helpers::read_file(&dane.file_path);
    let total_bytes = file_content.len();
    let bytes_counted: Vec<u32> = functions::count_bytes(&file_content);

    // println!("{:?}", bytes_counted);
    // bytes_counted.iter().enumerate().for_each(|(byte, value)| println!("{}: {}", byte as u8 as char, value));

    let probability: Vec<f64> = functions::probability(bytes_counted, total_bytes as u32);

    // println!("{:?}", probability);
    // probability.iter().enumerate().for_each(|(byte, value)| println!("{}: {}", byte as u8 as char, value));

    let cond_probability: Vec<Vec<f64>> = functions::conditional_probability(&file_content);

    let entropy: f64 = functions::entropy(&probability);

    let cond_entropy: f64 = functions::conditional_entropy(&probability, &cond_probability);

    if dane.table {
        let mut table = Table::new();
        table
            .set_header(vec!["Entropy", "Conditional Entropy"])
            .add_row(vec![format!("{}", entropy), format!("{}", cond_entropy)]);
        print!("{table}");
    } else {
        println!("entropy;conditional_entropy");
        println!("{};{}", entropy, cond_entropy);
    }
}
