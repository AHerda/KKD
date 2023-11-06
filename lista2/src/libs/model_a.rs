use std::ops::AddAssign;

use rayon::prelude::{IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator};

use super::model_metrics::ModelMetrics;

pub struct ModelA {
    cumulative_frequency: [usize; 258],
    pub bytes_processed: u64,
    frozen: bool,
    pub model_metrics: ModelMetrics,
}

impl ModelA {
    const CODE_VALUE_BITS: usize = ((usize::BITS + 3) / 2) as usize;
    const FREQUENCY_BITS: usize = usize::BITS as usize - ModelA::CODE_VALUE_BITS;

    pub fn new() -> ModelA {
        let mut cumulative_frequency = [0; 258];
        cumulative_frequency.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = i;
        });

        ModelA {
            cumulative_frequency,
            bytes_processed: 0,
            frozen: false,
            model_metrics: ModelMetrics::new(Self::CODE_VALUE_BITS, Self::FREQUENCY_BITS),
        }
    }

    fn pacify(&mut self) {
        self.bytes_processed += 1;
        // if self.bytes_processed % 1000 == 0 {
        //     print!("\r{}", self.bytes_processed);
        // }
    }

    fn frozen(&self) {
        println!("Frozen at: {}", self.bytes_processed);
    }

    fn update(&mut self, c: usize) {
        for i in c + 1..258 {
            self.cumulative_frequency[i] += 1;
        }
        if self.cumulative_frequency[257] >= self.model_metrics.max_freq {
            self.frozen();
            self.frozen = true;
        }
    }

    pub fn get_probability(&mut self, c: usize) -> Prob {
        let p = Prob {
            low: self.cumulative_frequency[c],
            high: self.cumulative_frequency[c + 1],
            count: self.cumulative_frequency[257],
        };
        if !self.frozen {
            self.update(c);
        }
        self.pacify();
        p
    }

    pub fn get_char(&mut self, scaled_value: usize) -> Result<(Prob, usize), ()> {
        self.pacify();
        for i in 0..257 {
            if scaled_value < self.cumulative_frequency[i + 1] {
                let c = i;
                let p = Prob {
                    low: self.cumulative_frequency[i],
                    high: self.cumulative_frequency[i + 1],
                    count: self.cumulative_frequency[257],
                };
                if !self.frozen {
                    self.update(c);
                }
                return Ok((p, c));
            }
        }
        Err(())
    }

    pub fn get_count(&self) -> usize {
        self.cumulative_frequency[257]
    }
}

#[derive(Debug)]
pub struct Prob {
    pub low: usize,
    pub high: usize,
    pub count: usize,
}

impl Prob {
    fn _range(&self) -> usize {
        self.high - self.low
    }
}

impl AddAssign<Prob> for Prob {
    fn add_assign(&mut self, other: Prob) {
        self.low += other.low;
        self.high += other.high;
        self.count += other.count;
    }
}
