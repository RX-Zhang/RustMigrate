
struct AptxPrediction {
    reconstructed_differences: Box<[i32]>,
    pos: usize,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> &mut i32 {
    let rd1 = &mut prediction.reconstructed_differences[..];
    let p = prediction.pos;

    let temp = rd1[p];
    rd1[p] = rd1[(p + order) % order];
    rd1[(p + order) % order] = temp;

    prediction.pos = (p + 1) % order;
    &mut rd1[(p + order) % order]
}
