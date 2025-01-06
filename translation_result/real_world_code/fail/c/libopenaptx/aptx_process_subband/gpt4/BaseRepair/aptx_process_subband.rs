
struct AptxPrediction {
    pos: i32,
    reconstructed_differences: Vec<i32>,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let p = prediction.pos as usize;
    let rd_index = (p + order as usize).wrapping_sub(1) % order as usize;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[rd_index];
    prediction.pos = (prediction.pos.wrapping_add(1)) % order;
    prediction.reconstructed_differences[p] = reconstructed_difference;
    &mut prediction.reconstructed_differences[p]
}
