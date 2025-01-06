
fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p) & !(((2u32 << p) - 1) as u32)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}
