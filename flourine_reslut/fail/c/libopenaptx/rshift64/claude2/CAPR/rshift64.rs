
use std::convert::TryInto;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << (shift.wrapping_sub(1) as i64);
    let mask: i64 = ((1i64 << (shift as i64 + 1)) - 1);
    let shifted = (value.wrapping_add(rounding) >> shift as i64).wrapping_sub(((value & mask) == rounding) as i64);
    shifted
}

