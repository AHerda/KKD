pub struct ModelMetrics {
    pub precision: usize,
    pub code_value_bits: usize,
    pub frequency_bits: usize,
    pub max_code: usize,
    pub max_freq: usize,
    pub one_fourth: usize,
    pub one_half: usize,
    pub three_fourths: usize,
}

impl ModelMetrics {
    pub const fn new(code_value_bits: usize, frequency_bits: usize) -> ModelMetrics {
        let precision = usize::BITS as usize;
        let max_code = (1 << code_value_bits) - 1;
        let max_freq = (1 << frequency_bits) - 1;
        let one_fourth = 1 << (code_value_bits - 2);
        let one_half = 2 * one_fourth;
        let three_fourths = 3 * one_fourth;

        ModelMetrics {
            precision,
            code_value_bits,
            frequency_bits,
            max_code,
            max_freq,
            one_fourth,
            one_half,
            three_fourths,
        }
    }

    pub fn dump(&self, name: &str) {
        println!("Model {} created with:", name);
        println!("CODE_VALUE of type usize with {} bits", self.precision);
        println!("CODE_VALUE_BITS {} bits giving MAX_CODE of {}", self.code_value_bits, self.max_code);
        println!("FREQUENCY_BITS {} bits giving MAX_FREQUENCY of {}", self.frequency_bits, self.max_freq);
        println!("MAX_CODE: {} (0x{:x})", self.max_code, self.max_code);
        println!("MAX_FREQ: {} (0x{:x})", self.max_freq, self.max_freq);
        println!("ONE_FOURTH: {} (0x{:x})", self.one_fourth, self.one_fourth);
        println!("ONE_HALF: {} (0x{:x})", self.one_half, self.one_half);
        println!("THREE_FOURTHS: {} (0x{:x})", self.three_fourths, self.three_fourths);
    }
}