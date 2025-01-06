
fn clip_intp2a(a: i32, p: u32) -> i32 {
    let mask = (1 << p) - 1;
    let a_u32 = if ((a as u64 + (1 << p) as u64) & !( (2 << p) - 1) as u64) != 0 {
        ((a as i64 >> 31) as u32).wrapping_shr(1) | mask
    } else {
        a as u32
    } & mask as u32;
    a_u32 as i32
}
