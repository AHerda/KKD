pub mod bitvec;

use std::cmp::{max, min};

use bitvec::Bit::{self, *};

pub fn hamming_encoding(data: Vec<Bit>) -> Result<Vec<Bit>, String> {
    if data.len() != 4 {
        return Err("Data too long".to_string());
    }
    let p1 = data[0] ^ data[1] ^ data[3];
    let p2 = data[0] ^ data[2] ^ data[3];
    let p3 = data[1] ^ data[2] ^ data[3];
    let p4 = data[0] ^ data[1] ^ data[2] ^ data[3] ^ p1 ^ p2 ^ p3;
    Ok(vec![p1, p2, data[0], p3, data[1], data[2], data[3], p4])
}

pub fn hamming_decoding(mut data: Vec<Bit>) -> Result<Vec<Bit>, Option<Vec<Bit>>> {
    if data.len() != 8 {
        return Err(None);
    }
    let p1 = data[0];
    let p2 = data[1];
    let d1 = data[2];
    let p3 = data[3];
    let d2 = data[4];
    let d3 = data[5];
    let d4 = data[6];
    let p4 = data[7];

    let p1_calc = d1 ^ d2 ^ d4;
    let p2_calc = d1 ^ d3 ^ d4;
    let p3_calc = d2 ^ d3 ^ d4;
    let p4_calc = d1 ^ d2 ^ d3 ^ d4 ^ p1 ^ p2 ^ p3;

    let mut error_pos = 0;
    if p1 != p1_calc {
        error_pos += 1;
    }
    if p2 != p2_calc {
        error_pos += 2;
    }
    if p3 != p3_calc {
        error_pos += 4;
    }

    if p4 != p4_calc {
        if error_pos == 0 {
            data[7] ^= One; // Correct the error in the parity bit
        } else {
            data[error_pos - 1] ^= One; // Correct the error
        }
    } else if error_pos != 0 {
        return Err(Some(vec![d1, d2, d3, d4])); // Double bit error detected
    }

    Ok(vec![data[2], data[4], data[5], data[6]]) // Return the data bits
}

pub fn string_slice_from_buffer(v: &[u8], i: usize, n: usize) -> String {
    String::from_utf8_lossy(&v[max(0, i - n)..min(v.len(), i + n)]).to_string()
}

pub fn color_not_matching(s1: String, s2: String) -> (String, String) {
    let mut colored1 = String::new();
    let mut colored2 = String::new();

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            colored1.push_str(&format!("{}", c1));
            colored2.push_str(&format!("{}", c2));
        } else {
            colored1.push_str(&format!("\x1b[41m{}\x1b[0m", c1));
            colored2.push_str(&format!("\x1b[41m{}\x1b[0m", c2));
        }
    }

    (colored1, colored2)
}
