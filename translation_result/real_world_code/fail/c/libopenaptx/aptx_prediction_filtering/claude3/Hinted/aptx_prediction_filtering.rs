
struct AptxPrediction {
    predicted_sample: i32,
    predicted_difference: i32,
    previous_reconstructed_sample: i32,
    s_weight: [i32; 2],
    d_weight: [i32; 4],
    reconstructed_differences: [i32; 8],
    pos: i32,
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
                                + prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = diffsign(reconstructed_difference, 0) * (1 << 23);
    let mut predicted_difference: i64 = 0;

    for i in 0..order {
        let idx = (prediction.pos as usize + order - i - 1) % order;
        let srd = (prediction.reconstructed_differences[idx] >> 31) | 1;
        prediction.d_weight[i] = prediction.d_weight[i].wrapping_sub(
            (prediction.d_weight[i].wrapping_sub(srd.wrapping_mul(srd0))) >> 8
        );
        predicted_difference = predicted_difference.wrapping_add(
            prediction.reconstructed_differences[idx] as i64 * prediction.d_weight[i] as i64
        );
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let p = prediction.pos as usize;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[p.wrapping_add(order)];
    prediction.pos = ((p + 1) % order) as i32;
    prediction.reconstructed_differences[prediction.pos as usize + order] = reconstructed_difference;
}

fn clip_intp2(value: i32, bits: i32) -> i32 {
    let max = (1 << (bits - 1)) - 1;
    let min = -(1 << (bits - 1));
    value.max(min).min(max)
}

fn diffsign(diff: i32, _zero: i32) -> i32 {
    if diff < 0 { -1 } else { 1 }
}
