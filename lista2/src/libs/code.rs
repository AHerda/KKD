// use crate::libs::model_a::Prob;
// use entropy;
// use rayon::prelude::*;

// const _MAX: u128 = u128::MAX;
// const MIN: u128 = u128::MIN;
// const BITS: usize = 126;
// const MAX: u128 = 1_u128 << BITS;
// const MAX_C: u128 = MAX - 1;
// const HALF_C: u128 = MAX >> 1;
// const QTR_C: u128 = HALF_C >> 1;
// const QTR3_C: u128 = HALF_C | QTR_C;

// pub fn encode(content: &[u8]) -> (Vec<bool>, u128, usize, usize) {
//     let mut high = MAX_C;
//     let mut low = MIN;
//     let m: usize = content.len();
//     let probability = entropy::probability(content); // "aaaaaaabcc".as_bytes());
//     let mut compressed_content: Vec<bool> = Vec::new();
//     let mut pending = 0_u32;
//     let mut d: u128;
//     let mut p: Prob;

//     println!("{:?}", probability);
//     for byte in content {
//         d = high - low;
//         p = Prob::sum_p(&probability, *byte as usize);
//         high = low + (p.upper * d as f64).round() as u128;
//         low = low + (p.lower * d as f64).round() as u128;

//         loop {
//             if high < HALF_C {
//                 low <<= 1;
//                 high <<= 1;

//                 output(&mut compressed_content, false, &mut pending);
//             } else if low > HALF_C {
//                 low <<= 1;
//                 high <<= 1;
//                 low -= MAX_C;
//                 high -= MAX_C;

//                 output(&mut compressed_content, true, &mut pending);
//             } else if low > QTR_C && high < QTR3_C {
//                 low <<= 1;
//                 high <<= 1;

//                 low -= HALF_C;
//                 high -= HALF_C;

//                 pending += 1;
//             } else {
//                 break;
//             }
//         }
//     }

//     let len = compressed_content.len();
//     (compressed_content, (high + low) >> 1, m, len)
// }

// fn output(compressed_content: &mut Vec<bool>, bit: bool, pending: &mut u32) {
//     compressed_content.push(bit);

//     while *pending > 0 {
//         compressed_content.push(!bit);
//         *pending -= 1;
//     }
// }

// pub fn decode(content: Vec<bool>) -> (Vec<u8>, usize) {
//     let mut high = MAX_C;
//     let mut low = MIN;
//     let mut value = 0_u128;
//     let mut decompressed: Vec<u8> = Vec::new();
//     let mut model = Model::new();
//     let mut d: u128;
//     let mut prob: (u128, u128, u128);
//     let mut scaled_value: u128;
//     let mut char: u16;
//     let mut content_iterator = content.iter();

//     for i in 0..BITS {
//         value <<= 1;
//         value += if *content_iterator.next().unwrap() {
//             1
//         } else {
//             0
//         };
//     }

//     loop {
//         d = high - low;
//         scaled_value = ((value - low) * model.count() as u128) / d;
//         (prob, char) = model.get_char(scaled_value).unwrap();

//         if char == 256 {
//             break;
//         }

//         decompressed.push(char as u8);
//         high = low + ((d * prob.1) / prob.2);
//         low = low + ((d * prob.0) / prob.2);

//         loop {
//             if high < HALF_C {
//             } else if low > HALF_C {
//                 value -= HALF_C;
//                 low -= HALF_C;
//                 high -= HALF_C;
//             } else if low > QTR_C && high < QTR3_C {
//                 value -= QTR_C;
//                 low -= QTR_C;
//                 high -= QTR_C;
//             } else {
//                 break;
//             }

//             low <<= 1;
//             high <<= 1;
//             high += 1;
//             value <<= 1;
//             value += if *content_iterator.next().unwrap() {
//                 1
//             } else {
//                 0
//             };
//         }
//     }

//     let len = decompressed.len();
//     (decompressed, len)
// }

// pub fn to_string(compressed: &[bool]) -> String {
//     compressed
//         .par_iter()
//         .map(|b| match b {
//             false => "0".to_string(),
//             true => "1".to_string(),
//         })
//         .collect::<String>()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn encode_abc() {
//         let content = "abc".as_bytes();
//         let (compressed, z, size, _size_compressed) = encode(&content);

//         println!("{z}");
//         println!("{:?}", compressed);
//         println!("{:?}", to_string(&compressed));

//         assert!(MIN <= z && z < MAX_C);
//         assert_eq!(size, 3);
//     }

//     fn encode2(content: &[u8], probability: &[f64]) -> (Vec<bool>, u128, usize) {
//         let mut high = MAX_C;
//         let mut low = MIN;
//         let m: usize = content.len();
//         let mut compressed_content: Vec<bool> = Vec::new();
//         let mut pending = 0_u32;
//         let mut d: u128;
//         let mut p: Prob;

//         println!("{:?}", probability);
//         for byte in content {
//             d = (high - low) + 1_u128;
//             p = Prob::sum_p(probability, *byte as usize);
//             high = low + (p.upper * d as f64).round() as u128;
//             low = low + (p.lower * d as f64).round() as u128;

//             loop {
//                 if high < HALF_C {
//                     low <<= 1;
//                     high <<= 1;

//                     output(&mut compressed_content, false, &mut pending);
//                 } else if low > HALF_C {
//                     low <<= 1;
//                     high <<= 1;
//                     low -= MAX_C;
//                     high -= MAX_C;

//                     output(&mut compressed_content, true, &mut pending);
//                 } else if low > QTR_C && high < QTR3_C {
//                     low <<= 1;
//                     high <<= 1;

//                     low -= HALF_C;
//                     high -= HALF_C;

//                     pending += 1;
//                 } else {
//                     break;
//                 }
//             }
//         }

//         (compressed_content, (high + low) >> 1, m)
//     }

//     pub fn decode2(content: Vec<bool>, tag: u128) -> (Vec<u8>, usize) {
//         let mut high = MAX_C;
//         let mut low = MIN;
//         let mut value = 0_u128;
//         let mut decompressed: Vec<u8> = Vec::new();
//         let mut model = Model::new();
//         let mut d: u128;
//         let mut prob: (u128, u128, u128);
//         let mut scaled_value: u128;
//         let mut char: u16;
//         let mut content_iterator = content.iter();
    
//         // for i in 0..BITS {
//         //     value <<= 1;
//         //     value += if *content_iterator.next().unwrap() {
//         //         1
//         //     } else {
//         //         0
//         //     };
//         // }
    
//         loop {
//             d = high - low;
//             scaled_value = ((value - low) * model.count() as u128) / d;
//             (prob, char) = model.get_char(scaled_value).unwrap();
    
//             if char == 256 {
//                 break;
//             }
    
//             decompressed.push(char as u8);
//             high = low + ((d * prob.1) / prob.2);
//             low = low + ((d * prob.0) / prob.2);
    
//             loop {
//                 if high < HALF_C {
//                 } else if low > HALF_C {
//                     value -= HALF_C;
//                     low -= HALF_C;
//                     high -= HALF_C;
//                 } else if low > QTR_C && high < QTR3_C {
//                     value -= QTR_C;
//                     low -= QTR_C;
//                     high -= QTR_C;
//                 } else {
//                     break;
//                 }
    
//                 low <<= 1;
//                 high <<= 1;
//                 high += 1;
//                 value <<= 1;
//                 value += if *content_iterator.next().unwrap() {
//                     1
//                 } else {
//                     0
//                 };
//             }
//         }
    
//         let len = decompressed.len();
//         (decompressed, len)
//     }
// }
