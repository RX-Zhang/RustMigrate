

use libc::int64_t;
use std::mem;

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 << (shift - 1)) as int64_t;
    let mask = ((1 << (shift + 1)) - 1) as int64_t;
    let shifted = (value.wrapping_add(rounding) >> shift) as int64_t;
    shifted.wrapping_sub((value & mask).wrapping_sub(rounding).wrapping_sub(0 as int64_t))
}

