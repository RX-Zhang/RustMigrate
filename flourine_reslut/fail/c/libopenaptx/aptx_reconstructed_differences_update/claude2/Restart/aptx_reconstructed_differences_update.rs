
use std::boxed::Box;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2], 
    d_weight: [i32; 24],
    pos: usize,
    reconstructed_differences: Box<[i32]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                         reconstructed_difference: i32,
                                         order: usize) -> &mut [i32] {
    
    let mut rd = prediction.reconstructed_differences.as_mut();
    let len = rd.len();
    let p = prediction.pos;

    let idx = (p + order) % order;
    
    let temp = rd[p];
    rd[p] = rd[idx];
    rd[idx] = temp;
    
    prediction.pos = (p + 1) % len;
    rd[prediction.pos] = reconstructed_difference;

    &mut rd[prediction.pos..]
}

