

use std::mem;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: Box<[i32]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let mut rd1 = mem::take(&mut prediction.reconstructed_differences);
    let rd1_len = rd1.len();
    let mut rd2 = vec![0; rd1_len].into_boxed_slice();

    let p = prediction.pos as usize;
    let order = order as usize;

    rd2[..order].copy_from_slice(&rd1[p..]);
    rd2[order..].copy_from_slice(&rd1[..p]);
    rd2[p] = reconstructed_difference;

    prediction.pos = prediction.pos.wrapping_add(1).rem_euclid(order as i32);
    prediction.reconstructed_differences = rd2;

    &mut prediction.reconstructed_differences[prediction.pos as usize]
}

