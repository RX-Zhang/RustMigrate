
#![feature(core_intrinsics)]

use std::intrinsics::wrapping_add;
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let max_value = Wrapping(1 << p);
    let clipped_value = wrapping_add(Wrapping(a), max_value);
    if clipped_value.0 & !((Wrapping(1i64 << (p + 1)) - Wrapping(1)).0 as i32) != 0 {
        (a >> 31) ^ (max_value.0 - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((wrapping_add(Wrapping(value), Wrapping(rounding))).0 >> shift)
        - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

