
fn opl__attenuation_increment(rate: u32) -> u32 {
    let shifted_rate = rate >> 1;
    shifted_rate.wrapping_add(shifted_rate)
}
