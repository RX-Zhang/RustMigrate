
fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction,
                                       reconstructed_difference: i32,
                                       order: i32) -> &mut i32 {
    let order = order as usize;
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order);
    let p = prediction.pos;

    rd1[p as usize] = rd2[p as usize];
    prediction.pos = p.wrapping_add(1) % order as i32;
    rd2[p as usize] = reconstructed_difference;
    &mut rd2[p as usize]
}

#[derive(Debug)]
struct AptxPrediction {
    reconstructed_differences: Box<[i32]>,
    pos: i32,
}

