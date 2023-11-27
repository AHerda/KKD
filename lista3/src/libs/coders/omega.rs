use crate::libs::bitvec::{
    Bit::{self, One, Zero},
    BitVec,
};
use crate::libs::coding_types::EOF;

pub fn omega_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    for &code in encodee {
        let mut code = code + 1;
        let mut bit_stack: Vec<Bit> = Vec::new();
        while code > 1 {
            let mut len = 0;
            let mut tmp = code;
            while tmp > 0 {
                len += 1;
                tmp >>= 1;
            }
            for i in 0..len {
                if (code >> i) & 1 == 1 {
                    bit_stack.push(One);
                } else {
                    bit_stack.push(Zero);
                }
            }
            code = len - 1;
        }
        bit_stack.iter().rev().for_each(|bit| bitvec.push(*bit));
        bitvec.push(Zero);
    }
    bitvec
}
pub fn omega_decode(decodee: &BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let decodee = decodee.clone().to_vector_of_bits();
    let mut idx = 0;

    while idx < decodee.len() {
        let mut bit = decodee[idx];
        let mut num = 1;
        while bit == One {
            idx += 1;
            bit = decodee[idx];
            let len = num;
            num = 1;
            for _ in 0..len {
                num <<= 1;
                if bit == One {
                    num |= 1;
                }
                idx += 1;
                bit = decodee[idx];
            }
        }
        if num - 1 == EOF {
            break;
        }
        result.push(num - 1);
        idx += 1;
    }
    result
}
