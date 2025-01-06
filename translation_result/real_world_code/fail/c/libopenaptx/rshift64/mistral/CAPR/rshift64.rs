
pub fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << ((shift as usize).checked_sub(1).unwrap_or(0) % 64);
    let mask = (1i64 << ((shift as usize).checked_add(1).unwrap_or(1) % 64)) - 1;
    ((value.wrapping_add(rounding)) >> (shift % 64)) - ((value & mask) == rounding) as i64
}
