
use std::convert::TryInto;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p % 32;
    let mask = (1 << (p + 1)) - 1;
    let rounding = (1 << p) - 1;

    if ((a as u32) + ((1 << p) as u32)) & !mask == 0 {
        a
    } else {
        (a >> 31) ^ rounding
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let shift = shift % 32;
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift + 1)) - 1) as i32;

    ((value.wrapping_add(rounding)) >> shift).wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23.try_into().unwrap())
}
