

use libc::int32_t;
use std::ops::Shr;

fn rshift32(value: int32_t, shift: u32) -> int32_t {
    let rounding = (1 << (shift - 1)) as int32_t;
    let mask = ((1 << (shift + 1)) - 1) as int32_t;
    let shifted = (value.wrapping_add(rounding)).wrapping_shr(shift);
    if (value & mask) == rounding as int32_t {
        shifted.wrapping_sub(1)
    } else {
        shifted
    }
}

