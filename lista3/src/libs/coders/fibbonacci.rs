use crate::libs::bitvec::{
    Bit::{self, One, Zero},
    BitVec,
};

struct Fibonacci {
    fib: Vec<usize>,
}
impl Fibonacci {
    fn new() -> Fibonacci {
        let fib: Vec<usize> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10_946, 17_711, 28_657, 46_368, 75_025, 121_393, 196_418, 317_811, 514_229, 832_040,
            1_346_269, 2_178_309, 3_524_578, 5_702_887, 9_227_465, 14_930_352, 24_157_817, 39_088_169,
        ];
        Fibonacci { fib }
    }
    fn largest_fib_leq(&self, n: usize) -> Option<usize> {
        if self.fib.last().unwrap() >= &n {
            for (i, f) in self.fib.iter().enumerate().rev() {
                if f <= &n {
                    return Some(i);
                }
            }
        }
        None
    }
}

pub fn fib_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    let fib = Fibonacci::new();
    for &code in encodee {
        let mut code = code + 1;
        let mut idx = fib.largest_fib_leq(code).unwrap();
        let mut bit_stack: Vec<Bit> = Vec::new();
        while code != 0 {
            bit_stack.push(One);
            code -= fib.fib[idx];
            let new_idx = fib.largest_fib_leq(code).unwrap();
            if new_idx == 0 {
                for _ in 2..idx {
                    bit_stack.push(Zero);
                }
                break;
            }
            for _ in 1..(idx - new_idx) {
                bit_stack.push(Zero);
            }
            idx = new_idx;
        }
        bit_stack.iter().rev().for_each(|&bit| bitvec.push(bit));
        bitvec.push(One);
    }

    bitvec
}
pub fn fib_decode(decodee: BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let mut bit_stack = Vec::new();
    let mut one_counter = 0;
    let fib = Fibonacci::new();
    for bit in decodee {
        if bit == One {
            one_counter += 1;
        } else {
            one_counter = 0;
        }
        if one_counter == 2 {
            let mut code = 0;
            for (i, &b) in bit_stack.iter().enumerate() {
                if b == One {
                    code += fib.fib[2 + i];
                }
            }
            result.push(code - 1);
            one_counter = 0;
            bit_stack.clear();
        } else {
            bit_stack.push(bit);
        }
    }
    result
}
