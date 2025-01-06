
fn rshift32(value: i32, shift: u32) -> i32 {
    let shift = (shift % 32) as u32; // Ensure the shift amount is within the valid range
    let rounding = 1i32.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i32.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    (value.wrapping_add(rounding).wrapping_shr(shift)) - ((value & mask) == rounding) as i32
}
