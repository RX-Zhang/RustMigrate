

use libc::int32_t;
use std::ops::Sub;

fn rshift32(value: int32_t, shift: u32) -> int32_t {
    let rounding = (1 << (shift - 1)) as int32_t;
    let mask = ((1 << (shift + 1)) - 1) as int32_t;
    let shifted_value = (value + rounding) >> shift;
    shifted_value.wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

