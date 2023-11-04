use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;

pub struct OutputBytes<T> {
    stream: T,
}

impl<T: Write> OutputBytes<T> {
    pub fn new(stream: T) -> Self {
        OutputBytes { stream }
    }

    pub fn put_byte(&mut self, c: u8) -> Result<usize> {
        self.stream.write(&[c])
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
