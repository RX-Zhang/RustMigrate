
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << (shift % 64 - 1);
    let mask: i64 = (1 << (shift % 64 + 1)) - 1;
    ((value.wrapping_add(rounding)) >> (shift % 64)) - ((value & mask) == rounding) as i64
}
