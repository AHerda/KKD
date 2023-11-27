use std::collections::HashMap;


use super::{
    bitvec::BitVec,
    coding_types::{CodingType, EOF},
};

#[derive(Default)]
pub struct Encoder {
    coding_type: CodingType,
}

impl Encoder {
    pub fn new(coding_type: CodingType) -> Encoder {
        Encoder { coding_type }
    }
    /// Implementation of LZW encodeing with the dictionary indices encoded via `coding_type`: [`CodeingType`] universal coding.
    pub fn encode(&self, encodee: &[u8], file: bool) -> BitVec {
        let mut indices = self._encode(encodee);
        if self.coding_type != CodingType::FIB && file {
            indices.push(EOF);
        }
        self.coding_type.encoode(&indices)
    }

    /// Implementation of LZW encodeing
    fn _encode(&self, encodee: &[u8]) -> Vec<usize> {
        let mut dictionary: HashMap<Vec<u8>, usize> =
            (0..256).map(|i| (vec![i as u8], i)).collect();

        let mut w = Vec::new();
        let mut result = Vec::new();

        for &b in encodee {
            let mut wc = w.clone();
            wc.push(b);

            if dictionary.contains_key(&wc) {
                w = wc;
            } else {
                result.push(dictionary[&w]);

                dictionary.insert(wc, dictionary.len());
                w.clear();
                w.push(b);
            }
        }

        if !w.is_empty() {
            result.push(dictionary[&w]);
        }

        result
    }
}
