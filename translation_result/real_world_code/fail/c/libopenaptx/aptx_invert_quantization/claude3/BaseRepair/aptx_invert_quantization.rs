
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a.wrapping_add(1) < (1 << p) as i32) & (a.wrapping_add(1) >= -(1 << p) as i32) {
        a
    } else {
        (a >> 31) ^ ((1 << p) - 1)
    }
}
