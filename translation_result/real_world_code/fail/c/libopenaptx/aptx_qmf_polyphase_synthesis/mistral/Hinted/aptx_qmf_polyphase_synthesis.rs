
fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 << (shift + 1)) - 1) as i64;
    let comparison = if (value & mask) == rounding { 1 } else { 0 };
    ((value + rounding) >> shift) - comparison
}
