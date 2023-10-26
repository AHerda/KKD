use entropy;
use rayon::prelude::*;

pub fn encode(content: &[u8]) -> (Vec<bool>, f64, usize) {
    let mut high = 1_f64;
    let mut low = 0_f64;
    let m: usize = content.len();
    let probabilities: Vec<f64> = entropy::probability(content);
    let mut compressed_content: Vec<bool> = Vec::new();
    let mut pending = 0_u32;

    // content.par_iter().for_each(|byte| {
    //     let d: f64 = high - low;
    //     high = low + (sum_p(&probabilities, (byte + 1) as usize) * d);
    //     low += d * sum_p(&probabilities, *byte as usize);
    // });

    println!("{:?}", probabilities);
    for byte in content {
        let d: f64 = high - low;
        high = low + (sum_p(&probabilities, (byte + 1) as usize) * d);
        low += d * sum_p(&probabilities, *byte as usize);

        loop {
            if high < 0.5 {
                low *= 2_f64;
                high *= 2_f64;

                compressed_content.push(false);

                while pending > 0 {
                    compressed_content.push(true);
                    pending -= 1;
                }
            } else if low > 0.5 {
                low = low * 2_f64 - 1_f64;
                high = high * 2_f64 - 1_f64;

                compressed_content.push(true);

                while pending > 0 {
                    compressed_content.push(false);
                    pending -= 1;
                }
            } else if low > 0.25 && high < 0.75 {
                low = low * 2_f64 - 0.5;
                high = high * 2_f64 - 0.5;
                pending += 1;
            } else {
                break;
            }
        }
    }

    (compressed_content, (high + low) / 2_f64, m)
}

fn sum_p(probabilities: &[f64], j: usize) -> f64 {
    probabilities[0..j].par_iter().sum()
}

pub fn to_string(compressed: &[bool]) -> String {
    compressed
        .par_iter()
        .map(|b| match b {
            false => "0".to_string(),
            true => "1".to_string(),
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_abc() {
        let content = "hello".as_bytes();
        let (compressed, z, m) = encode(&content);

        println!("{z}");
        println!("{:?}", compressed);
        println!("{:?}", to_string(&compressed));

        assert!(0.0 < z && z < 1.0);
        assert_eq!(m, 5);
    }
}
