

use libc::int32_t;
use libc::int64_t;
use std::mem;
use std::ops::Shr;

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        return ((a >> 31) & !((1 << p) - 1)) | ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
        let rounding = (1 << (shift - 1)) as int64_t;
        let mask = ((1 << (shift + 1)) - 1) as int64_t;
        return ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask).wrapping_sub(rounding));
    }

fn rshift64_clip24(value: int64_t, shift: u32) -> int32_t {
    return clip_intp2(value as int32_t, 23) as int32_t;
}

