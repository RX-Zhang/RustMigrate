

use std::boxed::Box;

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

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                         reconstructed_difference: i32,  
                                         order: i32) -> i32 {
    let mut p = prediction.pos;
    
    let rd1 = &mut *prediction.reconstructed_differences;
    
    let mut rd2 = rd1[(p + order) as usize % order as usize];
    std::mem::swap(&mut rd1[p as usize], &mut rd2);

    p = p.wrapping_add(1) % order;
    prediction.pos = p;

    rd2 = reconstructed_difference;

    rd2
}

