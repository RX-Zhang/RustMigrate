
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1) as i32;
    let mask = (1 << (shift + 1) as i32) - 1;
    ((value.wrapping_add(rounding)) >> shift as i32) - ((value & mask) == rounding) as i32
}
