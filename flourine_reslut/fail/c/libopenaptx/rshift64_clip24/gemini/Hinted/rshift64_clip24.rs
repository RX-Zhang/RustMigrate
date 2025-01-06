
use std::i32;
use std::i64;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p)) & !(((2 << p) - 1) as u32) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) & mask
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
