

use std::mem;
use std::usize;

const ROUNDING: i64 = (1 << 5) as i64;
const MASK24: i32 = ((1 << 24) - 1) as i32;

fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as usize) + (1 << p)) & !((2 << p) - 1) != 0 {
        return ((a >> 31) as i32) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    let shifted = (value.wrapping_add(rounding) >> shift) as i64;
    shifted - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64) -> i32 {
    let shifted = rshift64(value, 6);
    return clip_intp2((shifted & ((1 << 31) - 1)) as i32, 23);
}

