
use std::mem;

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

impl Default for AptxPrediction {
    fn default() -> Self {
        AptxPrediction {
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
}

pub fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> Box<i32> {
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order);
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = ((p as i32 + 1) % order as i32).wrapping_abs();
    let new_p = prediction.pos as usize;
    rd2[new_p] = reconstructed_difference;

    Box::new(rd2[new_p])
}
