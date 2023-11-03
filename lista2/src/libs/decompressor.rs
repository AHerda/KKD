use std::fs::File;
use std::io::Read;
use std::io::Write;

#[cfg(LOG)]
use std::fs::File;
#[cfg(LOG)]
use std::io::BufWriter;
#[cfg(LOG)]
use std::io::Write;

use super::byteio::InputBytes;
use super::byteio::OutputBytes;
use super::bitio::InputBits;
use super::model_a::ModelA;

pub struct Decompressor<R, W>
{
    m_input: InputBits<R>,
    m_output: OutputBytes<W>,
    m_model: ModelA,
}

impl<R: Read, W: Write> Decompressor<R, W>
{
    pub fn new(input: InputBits<R>, output: OutputBytes<W>, model: ModelA) -> Self {
        Decompressor {
            m_input: input,
            m_output: output,
            m_model: model,
        }
    }

    pub fn decompress(&mut self) -> std::io::Result<()> {
        #[cfg(LOG)]
        let mut log = BufWriter::new(File::create("decompressor.log")?);
        #[cfg(LOG)]
        writeln!(log, "{:x}", 0)?;

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
            self.m_output.put_byte(c as u8);

            #[cfg(LOG)]
            {
                writeln!(log, "{:x}", c)?;
                if c > 0x20 && c <= 0x7f {
                    writeln!(log, "({})", c as u8 as char)?;
                }
                writeln!(log, "{:x} {:x} =>", low, high)?;
            }

            high = low + (range * p.high) / p.count - 1;
            low = low + (range * p.low) / p.count;

            #[cfg(LOG)]
            writeln!(log, "{:x} {:x}", low, high)?;

            loop {
                if high < self.m_model.model_metrics.one_half {
                } else if low >= self.m_model.model_metrics.one_half {
                    value -= self.m_model.model_metrics.one_half;
                    low -= self.m_model.model_metrics.one_half;
                    high -= self.m_model.model_metrics.one_half;
                } else if low >= self.m_model.model_metrics.one_fourth && high < self.m_model.model_metrics.three_fourths {
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

        #[cfg(LOG)]
        writeln!(log, "{:x}", 256)?;
        #[cfg(LOG)]
        writeln!(log, "{:x} {:x}", low, high)?;

        Ok(())
    }
}

pub fn decompress(source: File, target: File, model: ModelA) -> std::io::Result<()>
{
    let in_bytes = InputBytes::new(source);
    let in_bits = InputBits::new(in_bytes, model.model_metrics.code_value_bits as i32);
    let out_bytes = OutputBytes::new(target);
    let mut d = Decompressor::new(in_bits, out_bytes, model);
    d.decompress()
}


