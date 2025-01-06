
fn rshift64(value: i64, shift: u32) -> i64 {
    let shift = shift % 64;
    if shift == 0 {
        return value.wrapping_sub(-9223372036854775808);
    }
    let rounding = 1i64.wrapping_shl(shift - 1);
    let mask = (1i64.wrapping_shl(shift + 1)).wrapping_sub(1);
    let result = value.wrapping_add(rounding).wrapping_shr(shift);
    result.wrapping_sub((value & mask == rounding) as i64)
}
