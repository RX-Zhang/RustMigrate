
#![feature(core_intrinsics)]

use std::intrinsics::*;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let max = Box::new((1i32 << p) - 1);
    let min = Box::new(-(1i32 << p));
    if (a as u32).wrapping_add((1u32 << p)) & !(((1u32 << (p + 1)) - 1) as u32) != 0 {
        if a < 0 { *min } else { *max }
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = Box::new(1i32 << (shift - 1));
    let mask = Box::new((1i32 << (shift + 1)) - 1);
    ((value.wrapping_add(*rounding) >> shift) as i32) - ((*mask == *rounding) as i32)  
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

