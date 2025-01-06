
fn clip_intp2(a: i32, p: u32) -> i32 {
    let shift = p % 32;
    if ((a as u32).wrapping_add(1u32 << shift)) & !((2u32 << shift).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << shift) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    if shift == 0 {
        return value;
    }
    let rounding = 1i32 << ((shift - 1) % 32);
    let mask = ((1i32 << (shift + 1) % 32)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> (shift % 32)) - (((value & mask) == rounding) as i32)
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
