

use libc::int32_t;

#[repr(C)]
struct AptxPrediction {
    prev_sign: [int32_t; 2],
    s_weight: [int32_t; 2],
    d_weight: [int32_t; 24],
    pos: int32_t,
    reconstructed_differences: [int32_t; 48],
    previous_reconstructed_sample: int32_t,
    predicted_difference: int32_t,
    predicted_sample: int32_t,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: int32_t,
    order: int32_t,
) -> &mut int32_t {
    let rd1 = &mut prediction.reconstructed_differences;
    let len = rd1.len();
    let order = order as usize;
    let p = prediction.pos as usize;

    let rd2 = &mut rd1[p + 1..len];
    rd2[0] = reconstructed_difference;

    prediction.pos = ((p as i32).wrapping_add(1)) % (order as i32);

    rd2.first_mut().unwrap()
}

