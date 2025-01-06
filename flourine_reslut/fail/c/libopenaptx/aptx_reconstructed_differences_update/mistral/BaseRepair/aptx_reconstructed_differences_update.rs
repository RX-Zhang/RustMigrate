

use libc::int32_t;

#[repr(C)]
struct AptxPrediction {
    prev_sign: [int32_t; 2],
    s_weight: [int32_t; 2],
    d_weight: [int32_t; 24],
    pos: int32_t,
    reconstructed_differences: Box<[int32_t; 48]>,
    previous_reconstructed_sample: int32_t,
    predicted_difference: int32_t,
    predicted_sample: int32_t,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: int32_t,
    order: int32_t,
) -> &mut int32_t {
    let rd2 = prediction.reconstructed_differences.as_mut_slice();
    let p = prediction.pos;

    prediction.pos = (p + 1) % order;
    let idx = (p % order) as usize;
    *rd2.get_mut(idx).expect("index within bounds") = rd2[idx].wrapping_add(reconstructed_difference);
    rd2.get_mut(idx).expect("index within bounds")
}

