use std::fs::File;
use std::io::Read;
use std::io::Write;

use super::bitio::InputBits;
use super::byteio::InputBytes;
use super::byteio::OutputBytes;
use super::model_a::ModelA;

pub struct Decompressor<R, W> {
    m_input: InputBits<R>,
    m_output: OutputBytes<W>,
    m_model: ModelA,
}

impl<R: Read, W: Write> Decompressor<R, W> {
    pub fn new(input: InputBits<R>, output: OutputBytes<W>, model: ModelA) -> Self {
        Decompressor {
            m_input: input,
            m_output: output,
            m_model: model,
        }
    }

    pub fn decompress(&mut self) -> std::io::Result<()> {
        let mut high = self.m_model.model_metrics.max_code;
        let mut low = 0;
        let mut value = 0;
        for _ in 0..self.m_model.model_metrics.code_value_bits {
            value <<= 1;
            value += if self.m_input.get_bit()? { 1 } else { 0 };
        }

        loop {
            let range = high - low + 1;
            let scaled_value = ((value - low + 1) * self.m_model.get_count()) / range;
            let (p, c) = self.m_model.get_char(scaled_value).expect("Error");
            if c == 256 {
                break;
            }
            let _ = self.m_output.put_byte(c as u8)?;

            high = low + (range * p.high) / p.count - 1;
            low = low + (range * p.low) / p.count;

            loop {
                if high < self.m_model.model_metrics.one_half {
                } else if low >= self.m_model.model_metrics.one_half {
                    value -= self.m_model.model_metrics.one_half;
                    low -= self.m_model.model_metrics.one_half;
                    high -= self.m_model.model_metrics.one_half;
                } else if low >= self.m_model.model_metrics.one_fourth
                    && high < self.m_model.model_metrics.three_fourths
                {
                    value -= self.m_model.model_metrics.one_fourth;
                    low -= self.m_model.model_metrics.one_fourth;
                    high -= self.m_model.model_metrics.one_fourth;
                } else {
                    break;
                }
                low <<= 1;
                high <<= 1;
                high += 1;
                value <<= 1;
                value += if self.m_input.get_bit()? { 1 } else { 0 };
            }
        }

        Ok(())
    }
}

pub fn decompress(source: File, target: File, model: ModelA) -> std::io::Result<()> {
    let in_bytes = InputBytes::new(source);
    let in_bits = InputBits::new(in_bytes, model.model_metrics.code_value_bits as i32);
    let out_bytes = OutputBytes::new(target);
    let mut d = Decompressor::new(in_bits, out_bytes, model);
    d.decompress()
}
