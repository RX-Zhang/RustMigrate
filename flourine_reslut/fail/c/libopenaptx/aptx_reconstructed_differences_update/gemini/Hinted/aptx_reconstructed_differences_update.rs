
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct AptxPrediction {
    pub prev_sign: [i32; 2],
    pub s_weight: [i32; 2],
    pub d_weight: [i32; 24],
    pub pos: i32,
    pub reconstructed_differences: [i32; 48],
    pub previous_reconstructed_sample: i32,
    pub predicted_difference: i32,
    pub predicted_sample: i32,
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
        &mut rd[p + order]
    }
}
