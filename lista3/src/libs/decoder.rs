use std::collections::HashMap;

use super::{bitvec::BitVec, coding_types::CodingType};

#[derive(Default)]
pub struct Decoder {
    coding_type: CodingType,
}

impl Decoder {
    pub fn new(coding_type: CodingType) -> Decoder {
        Decoder { coding_type }
    }
    /// Implementation of LZW decodein with the dictionary indices decoded via `coding_type`: [`CodeingType`] universal coding.
    pub fn decode(&self, decodee: BitVec) -> Vec<u8> {
        let indices = self.coding_type.decode(decodee);
        self._decode(&indices)
    }
    /// Implementation of LZW decodeing
    fn _decode(&self, mut decodee: &[usize]) -> Vec<u8> {
        let mut dictionary: HashMap<usize, Vec<u8>> =
            (0..256).map(|i| (i, vec![i as u8])).collect();

        let mut w = dictionary[&decodee[0]].clone();
        decodee = &decodee[1..];
        let mut decompressed = w.clone();

        for &k in decodee {
            let entry = if dictionary.contains_key(&k) {
                dictionary[&k].clone()
            } else if k == dictionary.len() {
                let mut entry = w.clone();
                entry.push(w[0]);
                entry
            } else {
                panic!("Invalid dictionary!");
            };

            decompressed.extend_from_slice(&entry);

            w.push(entry[0]);
            dictionary.insert(dictionary.len(), w);

            w = entry;
        }

        decompressed
    }
}
