
pub fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32).wrapping_add((1 << p) as u32) & !(((2 << p) as u32 - 1))) != 0 {
        (a >> 31) as i32 ^ ((1 << p) - 1)
    } else {
        a
    }
}
