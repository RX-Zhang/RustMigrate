
use std::boxed::Box;

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

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                         reconstructed_difference: i32,
                                         order: i32) -> &mut [i32] {
    
    let mut rd = &mut *prediction.reconstructed_differences;
    let p = prediction.pos;

    let idx = (p + order) % order;
    
    let temp = rd[p as usize];
    rd[p as usize] = rd[idx as usize];
    rd[idx as usize] = temp;
    
    prediction.pos = p.wrapping_add(1) % order;
    rd[p as usize] = reconstructed_difference;

    rd
}

