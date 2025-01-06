
use std::cmp::Ordering;

struct AptxPrediction {
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
    fn new() -> Self {
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

fn diffsign(x: i32, y: i32) -> i32 {
    match x.cmp(&y) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p)) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) -> i32 {
    let p = prediction.pos as usize;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[p + order];
    prediction.pos = ((p + 1) % order) as i32;
    let new_p = prediction.pos as usize;
    prediction.reconstructed_differences[new_p + order] = reconstructed_difference;
    prediction.reconstructed_differences[new_p + order]
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
        + prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let new_rd = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = diffsign(reconstructed_difference, 0) * (1 << 23);
    
    let mut predicted_difference: i64 = 0;
    for i in 0..order {
        let srd = (prediction.reconstructed_differences[prediction.pos as usize + order - i - 1] >> 31) | 1;
        prediction.d_weight[i] -= rshift32(prediction.d_weight[i] - srd * srd0, 8);
        predicted_difference += prediction.reconstructed_differences[prediction.pos as usize + order - i] as i64 * prediction.d_weight[i] as i64;
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
