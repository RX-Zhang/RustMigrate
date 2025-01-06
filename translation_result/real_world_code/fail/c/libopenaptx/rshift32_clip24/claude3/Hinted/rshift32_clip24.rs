
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let mask = (1u32 << p).wrapping_sub(1);
    if (a_u32.wrapping_add(1u32 << p) & !mask) != 0 {
        ((a >> 31) ^ ((1i32 << p) - 1)) as i32
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask == rounding) as i32)
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
