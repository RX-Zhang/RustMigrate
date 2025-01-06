
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
    let rd = &mut prediction.reconstructed_differences;
    let p = prediction.pos as usize;

    rd[p] = rd[(p + order as usize) % 48];
    prediction.pos = prediction.pos.wrapping_add(1).rem_euclid(order);
    rd[(prediction.pos as usize)] = reconstructed_difference;
    &mut rd[(prediction.pos as usize)]
}
