
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 as i64) << (shift + 1)) - 1;
    let shifted = ((value + rounding) >> shift);
    let clipped = if (value & mask) == rounding {
        shifted - 1
    } else {
        shifted
    };
    (clipped as i32).wrapping_shr(23)
}
