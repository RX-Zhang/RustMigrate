
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a as u32).wrapping_add(1) < (2u32 << p) {
        a
    } else {
        (a >> 31) ^ ((1 << p) - 1)
    }
}
