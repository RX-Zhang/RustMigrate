
struct AptxPrediction {
    reconstructed_differences: Box<[i32]>,
    pos: i32,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut Box<AptxPrediction>,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let p = prediction.pos as usize;
    let order_usize = order as usize;

    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order_usize);
    
    rd1[p] = rd2[p];
    prediction.pos = ((p + 1) % order_usize) as i32;
    rd2[p] = reconstructed_difference;
    &mut rd2[p]
}
