

use libc::int64_t;
use std::mem;

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 as int64_t) << (shift - 1);
    let mask = ((1 as int64_t) << (shift + 1)) - 1;
    let shifted = (value.wrapping_add(rounding)) >> shift;
    let masked = value & mask;
    let comparison = (masked == rounding as int64_t) as int64_t;
    shifted.wrapping_sub(comparison)
}

