use rayon::prelude::*;

pub fn count_bytes(content: &str) -> Vec<u32> {
    let mut count: Vec<u32> = vec![0; 256];

    for c in content.bytes() {
        count[c as usize] += 1;
    }

    count
}

pub fn probability(count_tab: Vec<u32>, total_count: u32) -> Vec<f64> {
    count_tab
        .par_iter()
        .map(|x| *x as f64 / total_count as f64)
        .collect()
}

pub fn conditional_probability(content: &str) -> Vec<Vec<f64>> {
    let bytes: Vec<u8> = content.bytes().collect();

    let mut cond_count_tab: Vec<Vec<i32>> = vec![vec![0; 256]; 256];
    let mut cond_probability: Vec<Vec<f64>> = vec![vec![0.0; 256]; 256];

    let mut prev_byte: u8 = bytes[0];
    bytes[1..].iter().for_each(|byte| {
        cond_count_tab[prev_byte as usize][*byte as usize] += 1;
        prev_byte = *byte;
    });

    cond_count_tab.iter().enumerate().for_each(|(i, v)| {
        cond_probability[i]
            .iter_mut()
            .enumerate()
            .for_each(|(j, cell)| *cell = (v[j] as f64) / (v.iter().sum::<i32>() as f64));
    });

    cond_probability
}
