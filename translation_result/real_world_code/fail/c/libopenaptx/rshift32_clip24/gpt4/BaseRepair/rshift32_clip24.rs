
fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32.wrapping_shl(p))) & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p % 32)) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl((shift.wrapping_sub(1) % 32));
    let mask = (1i32.wrapping_shl((shift.wrapping_add(1) % 32))) - 1;
    ((value.wrapping_add(rounding)).wrapping_shr(shift % 32)) - if (value & mask) == rounding { 1 } else { 0 }
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
