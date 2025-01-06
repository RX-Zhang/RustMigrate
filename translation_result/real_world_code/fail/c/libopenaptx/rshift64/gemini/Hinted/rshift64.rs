

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64
}
