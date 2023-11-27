use crate::libs::bitvec::{
    Bit::{One, Zero},
    BitVec,
};
use crate::libs::coding_types::EOF;

pub fn delta_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    for &code in encodee {
        let code = code + 1;

        let len = 1 + code.ilog2();
        let length_of_len = len.ilog2();

        for _ in 0..length_of_len {
            bitvec.push(Zero);
        }
        for i in (0..=length_of_len).rev() {
            if (len >> i) & 1 == 1 {
                bitvec.push(One);
            } else {
                bitvec.push(Zero);
            }
        }
        for i in (0..(len - 1)).rev() {
            if (code >> i) & 1 == 1 {
                bitvec.push(One);
            } else {
                bitvec.push(Zero);
            }
        }
    }
    bitvec
}
pub fn delta_decode(decodee: &BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let decodee = decodee.clone().to_vector_of_bits();
    let mut idx = 0;
    while idx < decodee.len() {
        let mut bit = decodee[idx];
        let mut num = 1;
        let mut len = 1;
        let mut length_of_len = 0;
        while bit == Zero {
            length_of_len += 1;
            idx += 1;
            bit = decodee[idx];
        }
        for _ in 0..length_of_len {
            len <<= 1;
            idx += 1;
            bit = decodee[idx];
            if bit == One {
                len |= 1;
            }
        }
        for _ in 0..(len - 1) {
            num <<= 1;
            idx += 1;
            bit = decodee[idx];
            if bit == One {
                num |= 1;
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
