
use std::cmp::min;

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
    pub fn new() -> Self {
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
    let p = prediction.pos as usize;
    let new_p = (p + 1) % order;

    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[p + order];
    prediction.reconstructed_differences[new_p + order] = reconstructed_difference;
    prediction.pos = new_p as i32;

    Box::new(prediction.reconstructed_differences[new_p + order])
}
