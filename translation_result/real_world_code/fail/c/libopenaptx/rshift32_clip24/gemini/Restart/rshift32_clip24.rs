
use std::convert::TryInto;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32: u32 = a.try_into().unwrap();
    let p_u32: u32 = p.try_into().unwrap();
    if (a_u32 + (1 << p_u32)) & !(2 << p_u32 - 1) != 0 {
        (a >> 31) ^ ((1 << p_u32) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift + 1)) - 1) as i32;
    ((value + rounding).wrapping_shr(shift as u32)) - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
