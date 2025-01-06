
use std::boxed::Box;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: usize,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                         reconstructed_difference: i32,
                                         order: i32)
                                         -> &mut [i32] {
    let rd1 = &mut prediction.reconstructed_differences;
    let p = prediction.pos;

    let idx = (p.wrapping_add(order as usize) % order as usize);
    rd1[idx] = reconstructed_difference;

    prediction.pos = p.wrapping_add(1);

    &mut rd1[idx..]
}

