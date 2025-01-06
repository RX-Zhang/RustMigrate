
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1i32.wrapping_shl(shift.saturating_sub(1) % 32);
    let mask: i32 = (1i32 << (shift.wrapping_add(1) % 32)) - 1;
    let wrapped_add = value.wrapping_add(rounding);
    let wrapped_shr = wrapped_add.wrapping_shr(shift % 32);
    wrapped_shr - (((value & mask) == rounding) as i32)
}
