use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub struct OutputBytes<T> {
    stream: T,
    outputed: usize,
}

impl<T: Write> OutputBytes<T> {
    pub fn new(stream: T) -> Self {
        OutputBytes { stream, outputed: 0 }
    }

    pub fn put_byte(&mut self, c: u8) -> Result<usize> {
        self.outputed += 1;
        self.stream.write(&[c])
    }

    pub fn outputed(&self) -> usize {
        self.outputed
    }
}

pub struct InputBytes<T> {
    stream: T,
}

impl<T: Read> InputBytes<T> {
    pub fn new(stream: T) -> Self {
        InputBytes { stream }
    }

    pub fn get_byte(&mut self) -> Result<usize> {
        let mut buf = [0_u8; 1];
        match self.stream.read(&mut buf) {
            Ok(0) => Err(ErrorKind::UnexpectedEof.into()),
            _ => Ok(buf[0].into())
        }
    }
}
