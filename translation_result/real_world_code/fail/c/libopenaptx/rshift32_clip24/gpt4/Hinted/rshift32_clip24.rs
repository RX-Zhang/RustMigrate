
fn clip_intp2(a: i32, p: u32) -> i32 {
    let u_a = a as u32;
    let one_p = 1u32.wrapping_shl(p);
    if u_a.wrapping_add(one_p) & !(one_p.wrapping_shl(1).wrapping_sub(1)) != 0 {
        (a >> 31) ^ (one_p - 1) as i32
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i32 << shift.wrapping_add(1)).wrapping_sub(1);
    ((value.wrapping_add(rounding) >> shift) as i32) - if (value & mask) == rounding { 1 } else { 0 }
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}
