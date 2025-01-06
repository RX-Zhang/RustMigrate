

use std::mem;

#[derive(Default)]
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

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1 << p) - 1;
    if (a.wrapping_add(1 << p) as u32) & !(((2 as u32).wrapping_shl(p)) - 1) != 0 {
        (a >> 31) ^ (mask as i32)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 as u32).wrapping_shl(shift + 1)) - 1;
    ((value.wrapping_add(rounding) >> shift) - ((value & mask as i32) == rounding) as i32)
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) {
    let p = prediction.pos as usize % order;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[order..].get(p).cloned().unwrap_or(0);
    prediction.pos = (p + 1) as i32;
    prediction.reconstructed_differences[order + p] = reconstructed_difference;
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );
    let predictor = clip_intp2(
        ((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
            + prediction.s_weight[1] as i64 * reconstructed_sample as i64)
            >> 22) as i32,
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = (reconstructed_difference.wrapping_sub(0).leading_zeros() == 0) as i32 * (1 << 23);
    let mut predicted_difference = 0;
    for i in 0..order {
        let srd = (prediction.reconstructed_differences[i] >> 31) | 1;
        prediction.d_weight[i] -= rshift32(
            prediction.d_weight[i].wrapping_sub(srd.wrapping_mul(srd0)),
            8,
        );
        predicted_difference += prediction.reconstructed_differences[i] as i64
            * prediction.d_weight[i] as i64;
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample =
        clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

