
use std::boxed::Box;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = (1i64 << (shift - 1)).wrapping_add(1);
    let mask: i64 = ((1i64 << (shift + 1)) - 1).wrapping_sub(1);
    let shifted = (value.wrapping_add(rounding) >> shift);
    let masked = (value & mask) == rounding;
    shifted.wrapping_sub(masked as i64)
}
