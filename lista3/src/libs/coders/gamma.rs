use crate::libs::bitvec::{
    Bit::{One, Zero},
    BitVec,
};
use crate::libs::coding_types::EOF;

pub fn gamma_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    let mut binary_rep = Vec::new();

    for code in encodee {
        let mut code = code + 1;
        binary_rep.clear();
        for _ in 0..(usize::BITS - 1 - code.leading_zeros()) {
            bitvec.push(Zero);
        }
        while code != 0 {
            binary_rep.push(code % 2);
            code /= 2;
        }
        binary_rep.iter().rev().for_each(|bit| {
            if *bit == 0 {
                bitvec.push(Zero);
            } else {
                bitvec.push(One);
            }
        });
    }
    bitvec
}
pub fn gamma_decode(decodee: &BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let mut counter = 0;
    let mut current_symbol: usize;
    let mut idx = 0;
    let decodee = decodee.clone().to_vector_of_bits();
    let mut bit = decodee[0];
    loop {
        // Counting zeros
        while bit == Zero {
            counter += 1;
            idx += 1;
            if idx >= decodee.len() - 1 {
                break;
            }
            bit = decodee[idx];
        }
        // outputng number
        current_symbol = 2_usize.pow(counter);
        for _ in 0..counter {
            idx += 1;
            if idx >= decodee.len() - 1 {
                break;
            }
            bit = decodee[idx];
            counter -= 1;
            if bit == One {
                current_symbol += 2_usize.pow(counter);
            }
        }
        if current_symbol - 1 == EOF {
            break;
        }
        result.push(current_symbol - 1);
        // reseting and checking break condition
        counter = 0;
        if idx >= decodee.len() - 1 {
            break;
        }
        idx += 1;
        bit = decodee[idx];
    }
    result
}
