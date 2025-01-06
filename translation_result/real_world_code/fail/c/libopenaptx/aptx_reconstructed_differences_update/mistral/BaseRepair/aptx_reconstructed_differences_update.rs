
use std::ops::Range;

#[repr(C)]
pub struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

pub fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let rd2 = &mut prediction.reconstructed_differences;
    let p = prediction.pos;

    let index = (p as usize % order as usize) as usize;
    rd2[index] = rd2[index].wrapping_add(reconstructed_difference);
    prediction.previous_reconstructed_sample = prediction.predicted_sample;
    prediction.predicted_sample = prediction.predicted_difference.wrapping_add(prediction.previous_reconstructed_sample);
    prediction.predicted_difference = rd2[(index + 1) % order as usize].wrapping_sub(prediction.predicted_sample);
    prediction.pos = (p + 1) % order;

    &mut rd2[index]
}
