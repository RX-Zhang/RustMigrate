

use libc::uint32_t;
use libc::int32_t;
use std::boxed;

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
    if ((a as uint32_t) + (1 << p)) & !((2 << p) - 1) != 0 {
        return ((a >> 31) as int32_t) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift32(value: int32_t, shift: u32) -> int32_t {
    let rounding = (1 << (shift - 1)) as int32_t;
    let mask = ((1 << (shift + 1)) - 1) as int32_t;
    return ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as int32_t;
}

fn rshift32_clip24(value: int32_t, shift: u32) -> int32_t {
    return clip_intp2(rshift32(value as int32_t, shift) as int32_t, 23);
}

