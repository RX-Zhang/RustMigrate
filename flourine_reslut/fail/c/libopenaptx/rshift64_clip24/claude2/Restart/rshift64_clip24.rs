
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let max = Wrapping(1_i32.wrapping_shl(p));
    let max = Wrapping(max.0 as i32); 
    let val = a.wrapping_add(max.0);
    if val & !(max.0 << 1 - 1) == 0 {
        if a >= 0 {
            max.0 - 1
        } else {
            -(max.0 as i32)
        }
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1_i64.wrapping_shl(shift - 1);
    let mask = (1_i64).wrapping_shl(shift + 1) - 1;
    let val = value.wrapping_add(rounding);
    let val = (val >> shift) - ((value & mask) == rounding) as i64;
    val
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

