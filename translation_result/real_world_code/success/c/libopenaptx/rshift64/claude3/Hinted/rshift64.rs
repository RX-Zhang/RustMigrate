
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i64.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    let shifted = value.wrapping_add(rounding).wrapping_shr(shift);
    let adjustment = ((value & mask) == rounding) as i64;
    shifted.wrapping_sub(adjustment)
}
