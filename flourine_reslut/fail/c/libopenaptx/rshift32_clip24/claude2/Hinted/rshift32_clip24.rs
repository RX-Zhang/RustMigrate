
#![feature(core_intrinsics)]

use std::{intrinsics::*, mem::size_of};

fn clip_intp2(a: i32, p: u32) -> i32 {
    let max: i32 = (1 << p) - 1; 
    let min: i32 = -(1 << p);
    a.max(min).min(max)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1 << (shift - 1);
    let mask: i32 = (1 << (shift + 1)) - 1;
    let shifted = (value.wrapping_add(rounding)).wrapping_shr(shift);
    let adjusted = shifted - (((value & mask) == rounding) as i32);
    adjusted  
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

