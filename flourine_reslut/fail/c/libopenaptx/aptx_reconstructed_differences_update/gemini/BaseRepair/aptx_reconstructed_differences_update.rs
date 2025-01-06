

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

impl AptxPrediction {
    pub fn reconstructed_differences_update(
        &mut self,
        reconstructed_difference: i32,
        order: usize,
    ) -> &mut i32 {
        let rd = &mut self.reconstructed_differences;
        let p = self.pos as usize;

        rd[p] = rd[p + order];
        self.pos = (self.pos + 1) % order as i32;
        rd[p + order] = reconstructed_difference;
        &mut rd[p + order]
    }
}

