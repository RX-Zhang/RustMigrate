
use std::mem;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let rd1 = &mut prediction.reconstructed_differences;
    let p = prediction.pos as usize;
    let new_p = (p + 1) % order as usize;

    rd1[p] = rd1[new_p];
    prediction.pos = prediction.pos.wrapping_add(1) % order;
    rd1[new_p] = reconstructed_difference;
    &mut rd1[new_p]
}
