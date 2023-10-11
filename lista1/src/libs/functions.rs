use std::result;

use rayon::prelude::*;

pub fn count_bytes(content: String) -> Vec<u32> {
    let mut count: Vec<u32> = vec![0; 256];

    for c in content.bytes() {
        count[c as usize] += 1;
    }

    count
}

pub fn probability(count_tab: Vec<u32>, total_count: u32) -> Vec<f64> {
    count_tab.par_iter().map(|x| {
        *x as f64 / total_count as f64
    }).collect()
}