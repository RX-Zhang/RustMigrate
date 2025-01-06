
fn rshift64(value: i64, shift: u32) -> i64 {
    let shift = shift % 64; // To ensure we don't shift more than the bits in value.
    let rounding: i64 = 1i64.wrapping_shl(shift - 1);
    let mask: i64 = (1i64.wrapping_shl(shift + 1)).wrapping_sub(1);
    
    // Perform the arithmetic shift right taking overflow into consideration
    let shifted_value = value.wrapping_add(rounding).wrapping_shr(shift as u32);

    // '-=' operation using wrapping_sub to avoid overflow
    shifted_value.wrapping_sub((value & mask == rounding) as i64)
}
