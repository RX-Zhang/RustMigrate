

use std::i32;

#[derive(Debug)]
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

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !(((2 << p) - 1) as u32) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 << (shift + 1)) - 1) as i32;
    return ((value.wrapping_add(rounding) >> shift) - ((value & mask) == rounding) as i32);
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> i32 {
    let reconstructed_differences = &mut *prediction.reconstructed_differences;
    let p = prediction.pos;

    reconstructed_differences[p] = reconstructed_differences[order..].to_vec().into_boxed_slice()[p];
    prediction.pos = (p + 1) % order;
    reconstructed_differences[order + p] = reconstructed_difference;
    return reconstructed_differences[order + p];
}

fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) {
    let reconstructed_sample =
        clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(
        ((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
            + prediction.s_weight[1] as i64 * reconstructed_sample as i64)
            >> 22) as i32,
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = (((reconstructed_difference >> 31) as i32) | 1) * (1 << 23);
    for i in 0..order {
        let srd = ((reconstructed_differences >> 31) | 1);
        prediction.d_weight[i] = prediction.d_weight[i]
            .wrapping_sub(rshift32(
                prediction.d_weight[i].wrapping_sub(srd * srd0),
                8,
            ));
        prediction.predicted_difference = prediction.predicted_difference.wrapping_add(
            ((reconstructed_differences as i64 * prediction.d_weight[i] as i64) >> 22).try_into().unwrap(),
        );
    }

    prediction.predicted_difference = clip_intp2(prediction.predicted_difference, 23);
    prediction.predicted_sample =
        clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

