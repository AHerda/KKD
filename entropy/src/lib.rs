use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn count_bytes(content: &[u8]) -> Vec<u32> {
    let mut count: Vec<u32> = vec![0; 256];

    for c in content {
        count[*c as usize] += 1;
    }

    count
}

pub fn probability(content: &[u8]) -> Vec<f64> {
    let total_count = content.len();
    let count_tab = count_bytes(content);
    count_tab
        .par_iter()
        .map(|x| *x as f64 / total_count as f64)
        .collect()
}

pub fn cumulative_probability(content: &[u8]) -> Vec<f64> {
    let mut c_probability: Vec<f64> = Vec::new();
    let probability = probability(content);
    c_probability.push(probability[0]);
    for i in 1..256 {
        c_probability.push(c_probability[i - 1] + probability[i]);
    }
    c_probability
}

pub fn conditional_probability(content: &[u8]) -> Vec<Vec<f64>> {
    let mut cond_count_tab: Vec<Vec<i32>> = vec![vec![0; 256]; 256];
    let mut cond_probability: Vec<Vec<f64>> = vec![vec![0.0; 256]; 256];

    let mut prev_byte: u8 = 0;
    content.iter().for_each(|byte| {
        cond_count_tab[prev_byte as usize][*byte as usize] += 1;
        prev_byte = *byte;
    });

    cond_count_tab.iter().enumerate().for_each(|(i, v)| {
        cond_probability[i]
            .iter_mut()
            .enumerate()
            .for_each(|(j, cell)| *cell = (v[j] as f64) / (v.par_iter().sum::<i32>() as f64));
    });

    cond_probability
}

pub fn entropy(content: &[u8]) -> f64 {
    let probability: Vec<f64> = probability(content);
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

fn entropy_help(probability: &Vec<f64>) -> f64 {
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
        0.0
    } else {
        x
    }
}

pub fn conditional_entropy(probability: &Vec<f64>, cond_probability: &[Vec<f64>]) -> f64 {
    let mut result: f64 = 0.;
    for i in 0..probability.len() {
        result += probability[i] * entropy_help(&cond_probability[i]);
    }

    result
}
