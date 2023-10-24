extern crate bit_vec;
extern crate byteorder;

use bit_vec::BitVec;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

struct AdaptiveArithmeticEncoder {
    low: u32,
    high: u32,
    pending_bits: u32,
    bit_buffer: BitVec,
}

impl AdaptiveArithmeticEncoder {
    fn new() -> Self {
        AdaptiveArithmeticEncoder {
            low: 0,
            high: 0xFFFF_FFFF,
            pending_bits: 0,
            bit_buffer: BitVec::new(),
        }
    }

    fn encode(&mut self, symbol: u8, model: &mut AdaptiveModel) {
        let total = model.total_count();
        let range = (self.high - self.low) / total as u32;

        let low = self.low + range * model.cumulative_count(symbol) as u32;
        let high = self.low + range * (model.cumulative_count(symbol) + 1) as u32;

        self.low = low;
        self.high = high;

        while (self.low & 0x8000_0000) == (self.high & 0x8000_0000) {
            self.shift_and_output();
        }

        model.update(symbol);
    }

    fn finish(&mut self) {
        for _ in 0..31 {
            self.shift_and_output();
        }
    }

    fn shift_and_output(&mut self) {
        let top_bit = (self.low & 0x8000_0000) >> 31;

        self.low = (self.low << 1) & 0x7FFF_FFFF;
        self.high = ((self.high << 1) & 0x7FFF_FFFF) | 1;

        self.bit_buffer.push(top_bit != 0);

        while self.pending_bits > 0 {
            self.bit_buffer.push(top_bit == 0);
            self.pending_bits -= 1;
        }
    }

    fn write_to_file(&mut self, output: &mut File) {
        while self.pending_bits > 0 {
            self.shift_and_output();
        }

        while self.bit_buffer.len() % 8 != 0 {
            self.bit_buffer.push(false);
        }

        let mut byte_buffer = BitVec::from_elem(8, false);
        for bit in &self.bit_buffer {
            byte_buffer.push(bit);
            if byte_buffer.len() == 8 {
                let byte: u8 = byte_buffer.iter().collect();
                output.write_u8(byte).unwrap();
                byte_buffer.clear();
            }
        }

        output.write_u32::<LittleEndian>(self.low).unwrap();
    }
}

struct AdaptiveModel {
    freq: HashMap<u8, u32>,
}

impl AdaptiveModel {
    fn new() -> Self {
        AdaptiveModel {
            freq: HashMap::new(),
        }
    }

    fn cumulative_count(&self, symbol: u8) -> u32 {
        let mut count = 0;
        for (&s, &f) in &self.freq {
            if s < symbol {
                count += f;
            }
        }
        count
    }

    fn total_count(&self) -> u32 {
        self.freq.values().sum()
    }

    fn update(&mut self, symbol: u8) {
        let count = self.freq.entry(symbol).or_insert(0);
        *count += 1;

        if self.total_count() > 65536 {
            self.freq.retain(|_, &mut v| v > 1);
        }
    }
}

fn main() {
    let input_file = "input.bin";
    let output_file = "output.bin";

    let mut input = File::open(input_file).expect("Failed to open input file");
    let mut output = File::create(output_file).expect("Failed to create output file");

    let mut encoder = AdaptiveArithmeticEncoder::new();
    let mut model = AdaptiveModel::new();

    let mut buffer = [0; 1];
    while input.read_exact(&mut buffer).is_ok() {
        let symbol = buffer[0];
        encoder.encode(symbol, &mut model);
    }

    encoder.finish();
    encoder.write_to_file(&mut output);
}
