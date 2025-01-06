
fn rshift32(value: i32, shift: u32) -> i32 {
    let shift = shift % 32;
    let rounding = 1 << (shift - 1);
    let mask = ((1 << shift) + 1) - 1; // Corrected the mask calculation
    let value_plus_rounding = value.wrapping_add(rounding as i32);
    let shifted = value_plus_rounding.wrapping_shr(shift as u32);

    if (value & mask as i32) == rounding as i32 {
        shifted - 1
    } else {
        shifted
    }
}
