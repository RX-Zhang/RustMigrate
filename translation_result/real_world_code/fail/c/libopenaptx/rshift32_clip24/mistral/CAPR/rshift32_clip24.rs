
pub fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p) & !(((2u32 << p) - 1u32))) != 0 {
        return (a >> 31) ^ ((1 << (p % 32)) - 1);
    } else {
        return a;
    }
}

pub fn rshift32(value: i32, shift: usize) -> i32 {
    let rounding = 1 << ((shift as u32).wrapping_sub(1) % 32);
    let mask = (1 << ((shift as u32).wrapping_add(1) % 32)) - 1;
    let result = ((value + rounding) >> (shift % 32)) - ((value & mask) == rounding) as i32;
    result
}

pub fn rshift32_clip24(value: i32, shift: usize) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
