
use std::convert::TryInto;

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: usize,
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

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p) & !((2u32 << p) - 1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i32)
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) -> Vec<i32> {
    let rd1 = &mut prediction.reconstructed_differences;
    let p = prediction.pos;

    rd1[p] = rd1[p + order];
    prediction.pos = (p + 1) % order;
    rd1[p + order] = reconstructed_difference;
    rd1.to_vec()
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64 + prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22).try_into().unwrap(), 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = DIFFSIGN(reconstructed_difference, 0) * ((1 << 23) as i32);
    let mut predicted_difference: i64 = 0;
    for i in 0..order {
        let srd = (reconstructed_differences[i].wrapping_sub(i as i32 + 1).wrapping_shr(31) | 1) as i32;
        let new_weight = rshift32(prediction.d_weight[i] - srd * srd0, 8);
        predicted_difference += reconstructed_differences[i] as i64 * prediction.d_weight[i] as i64;
        prediction.d_weight[i] = new_weight;
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor + prediction.predicted_difference, 23);
}

fn DIFFSIGN(x: i32, y: i32) -> i32 {
    ((x > y) as i32) - ((x < y) as i32)
}
