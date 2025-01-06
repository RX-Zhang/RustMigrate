
use std::ops::Add;

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
        order: i32,
    ) -> &mut i32 {
        let (rd1, rd2) = self.reconstructed_differences.split_at_mut(order as usize);
        let p = self.pos;

        rd1[p as usize] = rd2[p as usize];
        self.pos = p.wrapping_add(1) % order;
        rd2[p as usize] = reconstructed_difference;
        &mut rd2[p as usize]
    }
}
