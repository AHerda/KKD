use crate::functions;

pub fn zadanie(file_content: &Vec<u8>) -> (f64, f64) {
    let total_bytes = file_content.len();
    let bytes_counted: Vec<u32> = functions::count_bytes(file_content);
    let probability: Vec<f64> = functions::probability(bytes_counted, total_bytes as u32);
    let cond_probability: Vec<Vec<f64>> = functions::conditional_probability(file_content);
    let entropy: f64 = functions::entropy(&probability);
    let cond_entropy: f64 = functions::conditional_entropy(&probability, &cond_probability);

    (entropy, cond_entropy)
}
