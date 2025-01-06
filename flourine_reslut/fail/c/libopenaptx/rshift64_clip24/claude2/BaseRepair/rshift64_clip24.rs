
use std::mem::size_of;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let max = (1i32 << p) - 1;
    let min = -(1i32 << p);
    if a > max { max }
    else if a < min { min }
    else { a }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    let shifted = (value.wrapping_add(rounding)) >> shift;
    shifted - (shifted & mask == rounding as i64) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)  
}
