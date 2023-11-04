use std::fs::File;
use std::io::ErrorKind;
use std::io::Result;
use std::io::{Read, Write};

use super::bitio::OutputBits;
use super::byteio::{InputBytes, OutputBytes};
use super::model_a::ModelA;

struct Compressor<R: Read, W: Write> {
    input: InputBytes<R>,
    output: OutputBits<W>,
    model: ModelA,
}

impl<R: Read, W: Write> Compressor<R, W> {
    fn new(input: R, output: W, model: ModelA) -> Self {
        Compressor {
            input: InputBytes::new(input),
            output: OutputBits::new(OutputBytes::new(output)),
            model,
        }
    }

    fn compress(&mut self) -> Result<()> {
        let mut pending_bits = 0;
        let mut low = 0;
        let mut high = self.model.model_metrics.max_code;
        loop {
            let c = match self.input.get_byte() {
                Ok(byte) => byte,
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        256
                    } else {
                        return Err(e);
                    }
                }
            };
            let p = self.model.get_probability(c);
            let range = high - low + 1;

            high = low + (range * p.high / p.count) - 1;
            low = low + (range * p.low / p.count);
            loop {
                if high < self.model.model_metrics.one_half {
                    self.put_bit_plus_pending(0, &mut pending_bits)?;
                }
                else if low >= self.model.model_metrics.one_half {
                    self.put_bit_plus_pending(1, &mut pending_bits)?;
                }
                else if low >= self.model.model_metrics.one_fourth
                    && high < self.model.model_metrics.three_fourths {
                    pending_bits += 1;
                    low -= self.model.model_metrics.one_fourth;
                    high -= self.model.model_metrics.one_fourth;
                } 
                else {
                    break;
                }

                high <<= 1;
                high += 1;
                low <<= 1;
                high &= self.model.model_metrics.max_code;
                low &= self.model.model_metrics.max_code;
            }
            if c == 256 {
                break;
            }
        }
        pending_bits += 1;
        if low < self.model.model_metrics.one_fourth {
            self.put_bit_plus_pending(0, &mut pending_bits)?;
        } else {
            self.put_bit_plus_pending(1, &mut pending_bits)?;
        }
        // self.output.flush()?; // What this function supposed to do???
        Ok(())
    }

    fn put_bit_plus_pending(&mut self, bit: u8, pending_bits: &mut u8) -> Result<()> {
        self.output.put_bit(bit != 0)?;
        for _ in 0..*pending_bits {
            self.output.put_bit(bit == 0)?;
        }
        *pending_bits = 0;
        Ok(())
    }
}

pub fn compress(source: File, target: File, model: ModelA) -> Result<()> {
    let mut compressor = Compressor::new(source, target, model);
    compressor.compress()
}
