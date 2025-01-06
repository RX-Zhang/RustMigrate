
pub struct AptxPrediction {
    pub reconstructed_differences: Vec<i32>,
    pub pos: i32,
}

pub fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut [i32] {
    let mut rd1 = &mut prediction.reconstructed_differences;
    let (rd2, rd3) = rd1.split_at_mut(order as usize);
    let p = prediction.pos;

    let temp = rd3[p as usize];
    let rd4 = &mut rd3[0..1];
    rd4[0] = temp;
    prediction.pos = (p + 1) % order;
    rd3[prediction.pos as usize] = reconstructed_difference;
    return &mut rd3[prediction.pos as usize..];
}
