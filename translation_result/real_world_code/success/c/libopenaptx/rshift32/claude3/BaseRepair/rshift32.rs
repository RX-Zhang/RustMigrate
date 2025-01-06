
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1_i32.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1_i32.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    let shifted = value.wrapping_add(rounding).wrapping_shr(shift);
    let adjustment = if value & mask == rounding { 1 } else { 0 };
    shifted.wrapping_sub(adjustment)
}
