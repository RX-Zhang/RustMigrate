
fn clip_intp2(a: i32, p: u32) -> i32 {
    let uint_a = a as u32;
    let shift_p = 1u32.wrapping_shl(p);
    if uint_a.wrapping_add(shift_p) & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p)) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift - 1);
    let mask = (1i64.wrapping_shl(shift.wrapping_add(1))).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift)
        - if (value & mask) == rounding { 1 } else { 0 }
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
