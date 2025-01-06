
fn rshift64(value: i64, shift: u32) -> i64 {
    // Adjust the shift to avoid overflow issues
    let shift = shift % 64;

    if shift == 0 {
        return value;
    }

    let rounding = 1i64.wrapping_shl(shift - 1);
    let mask = (1i64.wrapping_shl(shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)).wrapping_shr(shift))
        .wrapping_sub(((value & mask) == rounding) as i64)
}
