use super::{
    bitvec::BitVec,
    coders::{
        delta::{delta_decode, delta_encode},
        fibbonacci::{fib_decode, fib_encode},
        gamma::{gamma_decode, gamma_encode},
        omega::{omega_decode, omega_encode},
    },
};

pub const EOF: usize = usize::MAX - 1;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CodingType {
    #[default]
    OMEGA,
    GAMMA,
    DELTA,
    FIB,
}

impl CodingType {
    pub fn from_str(from: &str) -> Option<CodingType> {
        match from {
            "gamma" => Some(CodingType::GAMMA),
            "delta" => Some(CodingType::DELTA),
            "fib" => Some(CodingType::FIB),
            "omega" => Some(CodingType::OMEGA),
            _ => None,
        }
    }

    pub fn encoode(&self, encodee: &Vec<usize>) -> BitVec {
        match self {
            CodingType::OMEGA => omega_encode(encodee),
            CodingType::GAMMA => gamma_encode(encodee),
            CodingType::DELTA => delta_encode(encodee),
            CodingType::FIB => fib_encode(encodee),
        }
    }
    pub fn decode(&self, decodee: BitVec) -> Vec<usize> {
        match self {
            CodingType::OMEGA => omega_decode(&decodee),
            CodingType::GAMMA => gamma_decode(&decodee),
            CodingType::DELTA => delta_decode(&decodee),
            CodingType::FIB => fib_decode(decodee),
        }
    }
}
