
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = ((1i64 << (shift + 1)) - 1) as i64;
    let result = ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64;
    result
}
