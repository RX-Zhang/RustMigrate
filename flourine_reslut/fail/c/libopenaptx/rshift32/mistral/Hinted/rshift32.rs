

use libc::int32_t;
use std::ops::Shr;

fn rshift32(value: int32_t, shift: u32) -> int32_t {
    let rounding = (1 << (shift - 1)) as int32_t;
    let mask = ((1 << (shift + 1)) - 1) as int32_t;
    let shifted = (value.wrapping_add(rounding)).shr(shift);
    let masked = value & mask;
    let comparison = (masked == rounding as int32_t) as i32;
    shifted.wrapping_sub(comparison)
}

