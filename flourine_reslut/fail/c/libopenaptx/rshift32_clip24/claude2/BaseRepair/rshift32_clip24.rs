
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1u32 << p).wrapping_sub(1);
    let overflow = (a as u32).wrapping_add(1u32 << p) & !mask;
    if overflow != 0 { 
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    let shifted = (Wrapping(value).0 + rounding) >> shift;
    shifted - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
