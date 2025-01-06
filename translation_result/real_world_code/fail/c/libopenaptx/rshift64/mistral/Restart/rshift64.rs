
pub fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    let shifted_value = (value.wrapping_add(rounding) >> shift) as i64;
    let masked_value = (value & mask) as i64;
    shifted_value - ((masked_value == rounding) as i64)
}
