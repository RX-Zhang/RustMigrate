
use std::convert::TryInto;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: usize,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

impl AptxPrediction {
    fn new() -> Self {
        Self {
            prev_sign: [0; 2],
            s_weight: [0; 2],
            d_weight: [0; 24],
            pos: 0,
            reconstructed_differences: [0; 48],
            previous_reconstructed_sample: 0,
            predicted_difference: 0,
            predicted_sample: 0,
        }
    }

    fn clip_intp2(a: i32, p: u32) -> i32 {
        if (((a as u32).wrapping_add(1u32 << p)) & !(((2u32) << p) - 1)) != 0 {
            (a >> 31) ^ ((1 << p) - 1)
        } else {
            a
        }
    }

    fn rshift32(value: i32, shift: u32) -> i32 {
        let rounding = 1 << (shift - 1);
        let mask = ((1 << (shift + 1)) - 1) as i32;
        ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
    }

    fn aptx_reconstructed_differences_update(&mut self, reconstructed_difference: i32, order: usize) -> i32 {
        let p = self.pos;
        self.pos = (p + 1) % self.reconstructed_differences.len();
        self.reconstructed_differences[p] = reconstructed_difference;
        reconstructed_difference
    }

    fn aptx_prediction_filtering(&mut self, reconstructed_difference: i32, order: usize) {
        let reconstructed_sample = Self::clip_intp2(reconstructed_difference.wrapping_add(self.predicted_sample), 23);
        let predictor = Self::clip_intp2(((self.s_weight[0] as i64 * self.previous_reconstructed_sample as i64
            + self.s_weight[1] as i64 * reconstructed_sample as i64) >> 22).try_into().unwrap(), 23);
        self.previous_reconstructed_sample = reconstructed_sample;

        let _ = self.aptx_reconstructed_differences_update(reconstructed_difference, order);
        let srd0 = (Self::diffsign(reconstructed_difference, 0) * ((1 << 23) as i32)) as i32;
        let mut predicted_difference: i64 = 0;
        for i in 0..order {
            let srd = (self.reconstructed_differences[order - i - 1].wrapping_shr(31) | 1) as i32;
            self.d_weight[i] -= Self::rshift32(self.d_weight[i] - srd * srd0, 8);
            predicted_difference += self.reconstructed_differences[order - i] as i64 * self.d_weight[i] as i64;
        }

        self.predicted_difference = Self::clip_intp2((predicted_difference >> 22) as i32, 23);
        self.predicted_sample = Self::clip_intp2(predictor + self.predicted_difference, 23);
    }

    fn diffsign(x: i32, y: i32) -> i32 {
        ((x > y) as i32) - ((x < y) as i32)
    }
}
