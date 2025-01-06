
struct AptxPrediction {
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
    fn aptx_reconstructed_differences_update(&mut self, reconstructed_difference: i32, order: usize) -> &mut i32 {
        let rd1 = &mut self.reconstructed_differences;
        let (left, right) = rd1.split_at_mut(order);
        let p = self.pos as usize;

        left[p] = right[p];
        self.pos = ((self.pos + 1) % order as i32).wrapping_add(0);
        right[self.pos as usize] = reconstructed_difference;
        &mut right[self.pos as usize]
    }
}
