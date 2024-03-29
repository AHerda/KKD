use std::ops::{BitXor, BitXorAssign};

use Bit::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn to_u8(&self) -> u8 {
        match self {
            Zero => 0,
            One => 1,
        }
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, Zero) => Zero,
            (One, One) => Zero,
            (Zero, One) => One,
            (One, Zero) => One,
        }
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

/// BitVector Implementation
/// # Fields
/// - `vec`: vector of bytes
/// - `inside_idx`: index of the next bit to be pushed
/// - `outside_idx`: index of the current byte
/// - `inside_itr_couner`: index of the next bit to be outputed
/// - `outside_itr_couner`: index of the current byte for iterateing
#[derive(Debug, Clone)]
pub struct BitVec {
    vec: Vec<u8>,
    inside_idx: usize,
    outside_idx: usize,
    inside_itr_couner: usize,
    outside_itr_couner: usize,
}

impl Iterator for BitVec {
    type Item = Bit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inside_itr_couner == 8 {
            self.inside_itr_couner = 0;
            self.outside_itr_couner += 1;
        }
        if self.outside_idx < self.outside_itr_couner {
            self.inside_itr_couner = 0;
            self.outside_itr_couner = 0;
            return None;
        }
        if self.outside_idx == self.outside_itr_couner && self.inside_idx == self.inside_itr_couner
        {
            self.inside_itr_couner = 0;
            self.outside_itr_couner = 0;
            return None;
        }
        let output: Bit =
            if (self.vec[self.outside_itr_couner] & (0x80 >> self.inside_itr_couner)) > 0 {
                One
            } else {
                Zero
            };
        self.inside_itr_couner += 1;
        Some(output)
    }
}

impl BitVec {
    /// Creates a new [BitVec] with initialised first byte
    pub fn new() -> BitVec {
        BitVec {
            vec: vec![0],
            inside_idx: 0,
            outside_idx: 0,
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }
    /// Creates a [BitVec] from a vector of bytes
    pub fn from_bytes(bytes: Vec<u8>) -> BitVec {
        BitVec {
            vec: bytes.clone(),
            inside_idx: 0,
            outside_idx: bytes.len(),
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }
    /// Outputs a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.vec.clone()
    }
    /// Outputs a vector of [Bit]s
    pub fn to_vector_of_bits(&mut self) -> Vec<Bit> {
        let mut vector_of_bits = Vec::new();
        for bit in self {
            vector_of_bits.push(bit);
        }
        vector_of_bits
    }
    /// Pushes the supplied `bit`:[Bit] to our vector
    pub fn push(&mut self, bit: Bit) {
        if self.inside_idx == 8 {
            self.inside_idx = 0;
            self.outside_idx += 1;
            self.vec.push(0);
        }
        match bit {
            Bit::Zero => (),
            Bit::One => self.vec[self.outside_idx] ^= 0x80 >> self.inside_idx,
        }
        self.inside_idx += 1;
    }
    pub fn push_byte(&mut self, byte: u8) {
        for n in (0..8).rev() {
            if (byte & (1 << n)) > 0 {
                self.push(One);
            } else {
                self.push(Zero)
            }
        }
    }
    pub fn push_k_lsb(&mut self, byte: u8, k: u8) {
        for n in (0..k).rev() {
            if (byte & (1 << n)) > 0 {
                self.push(One);
            } else {
                self.push(Zero)
            }
        }
    }
}
