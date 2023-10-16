use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


pub fn count_bytes(content: &Vec<u8>) -> Vec<u32> {
    let mut count: Vec<u32> = vec![0; 256];

    for c in content {
        count[*c as usize] += 1;
    }

    count
}

pub fn probability(count_tab: Vec<u32>, total_count: u32) -> Vec<f64> {
    count_tab
        .par_iter()
        .map(|x| *x as f64 / total_count as f64)
        .collect()
}

pub fn conditional_probability(bytes: &[u8]) -> Vec<Vec<f64>> {
    let mut cond_count_tab: Vec<Vec<i32>> = vec![vec![0; 256]; 256];
    let mut cond_probability: Vec<Vec<f64>> = vec![vec![0.0; 256]; 256];

    let mut prev_byte: u8 = 0;
    bytes.iter().for_each(|byte| {
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

pub fn entropy(probability: &Vec<f64>) -> f64 {
    let x: f64 = probability
        .par_iter()
        .fold(
            || 0.,
            |sum, x| {
                if *x == 0. {
                    sum
                } else {
                    sum + (x * (1. / x).log2())
                }
            },
        )
        .sum();
    if x.is_nan() {
        0.
    } else {
        x
    }
}

pub fn conditional_entropy(probability: &Vec<f64>, cond_probability: &[Vec<f64>]) -> f64 {
    let mut result: f64 = 0.;
    for i in 0..probability.len() {
        result += probability[i] * entropy(&cond_probability[i]);
    }

    result
}
