
fn rshift64(value: i64, shift: u32) -> i64 {
    // Guard against shifting more than the number of bits in i64
    let shift = shift % 64;
    // Handle the case where shift is 0 to avoid underflow in the next step
    if shift == 0 {
        return value;
    }
    let rounding = 1i64.wrapping_shl(shift - 1);
    let mask = (1i64.wrapping_shl(shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift) - (((value & mask) == rounding) as i64)
}
