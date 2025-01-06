

use libc::int32_t;
use libc::uint32_t;
use std::boxed;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !((2 << p) - 1)) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    if shift > 31 {
        return 0;
    }
    value >> shift
}

