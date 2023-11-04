use std::io::{Read, Result, Write, ErrorKind};

use super::byteio::{InputBytes, OutputBytes};

pub struct OutputBits<T> {
    output: OutputBytes<T>,
    next_byte: u8,
    mask: u8,
}

impl<T: Write> OutputBits<T> {
    pub fn new(output: OutputBytes<T>) -> Self {
        OutputBits {
            output,
            next_byte: 0,
            mask: 0x80,
        }
    }

    pub fn put_bit(&mut self, val: bool) -> Result<usize> {
        if val {
            self.next_byte |= self.mask;
        }
        self.mask >>= 1;
        if self.mask == 0 {
            self.output.put_byte(self.next_byte)?;
            self.mask = 0x80;
            self.next_byte = 0;
        }
        Ok(0)
    }
}

pub struct InputBits<T> {
    input: InputBytes<T>,
    current_byte: i32,
    last_mask: u8,
    code_value_bits: i32,
}

impl<T: Read> InputBits<T> {
    pub fn new(input: InputBytes<T>, code_value_bits: i32) -> Self {
        InputBits {
            input,
            current_byte: 0,
            last_mask: 1,
            code_value_bits,
        }
    }

    pub fn get_bit(&mut self) -> Result<bool> {
        if self.last_mask == 1 {
            self.current_byte = self.input.get_byte().unwrap_or(0) as i32;
            if self.current_byte < 0 {
                if self.code_value_bits <= 0 {
                    return Err(ErrorKind::UnexpectedEof.into());
                } else {
                    self.code_value_bits -= 8;
                }
            }
            self.last_mask = 0x80;
        } else {
            self.last_mask >>= 1;
        }
        Ok((self.current_byte & self.last_mask as i32) != 0)
    }
}
