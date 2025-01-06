
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1 << ((shift % 32) - 1);
    let mask: i32 = (1 << ((shift % 32) + 1)) - 1;
    ((value + rounding).wrapping_shr(shift % 32)) - ((value & mask) == rounding) as i32
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
