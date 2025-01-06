

use libc::int32_t;
use std::mem;
use std::ops::Shr;

const DIFFSIGN: fn(i32, i32) -> i32 = {
    |x, y| ((x > y) as i32) - ((x < y) as i32)
};

#[repr(C)]
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

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift + 1)) - 1) as i32;
    ((value.wrapping_add(rounding)).shr(shift) as i32) - (((value & mask) == rounding) as i32)
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                         reconstructed_difference: i32,
                                         order: i32) -> &mut i32 {
    let rd2 = &mut prediction.reconstructed_differences[..order as usize];
    let p = prediction.pos;

    rd2[p as usize] = reconstructed_difference;
    prediction.pos = (p + 1) % order;
    &mut rd2[p as usize]
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction,
                              reconstructed_difference: i32,
                              order: i32) {
    let mut reconstructed_sample: i32;
    let mut predictor: i32;
    let mut srd0: i32;
    let mut srd: i32;
    let mut predicted_difference: i64 = 0;
    let mut rd2 = &mut prediction.reconstructed_differences[..order as usize];

    reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    predictor = clip_intp2(((prediction.s_weight[0] as i64).wrapping_mul(prediction.previous_reconstructed_sample as i64) as i32).wrapping_add((prediction.s_weight[1] as i64).wrapping_mul(reconstructed_sample as i64) as i32 >> 22), 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    srd0 = (DIFFSIGN(reconstructed_difference, 0) * (1 << 23)) as i32;
    for i in 0..order {
        let idx = (i + 1) as isize * -1 - 1;
        srd = ((rd2[idx as usize] >> 31) as i32) | 1;
        prediction.d_weight[i as usize] -= rshift32(prediction.d_weight[i as usize] - (srd * srd0), 8);
        predicted_difference += (rd2[idx as usize] as i64) * (prediction.d_weight[i as usize] as i64);
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

