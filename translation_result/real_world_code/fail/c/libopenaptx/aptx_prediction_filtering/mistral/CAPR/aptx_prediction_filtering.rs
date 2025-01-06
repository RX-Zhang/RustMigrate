
fn clip_intp2(a: i32, p: usize) -> i32 {
    let a_u = a as u32;
    let p_u = p as u32;
    let mask = ((1 << (p_u + 1)) - 1) as i32;
    if (a_u + (1 << p_u)) & !((2 << p_u) - 1) != 0 {
        ((a >> 31) ^ 1) << p_u - 1
    } else {
        a
    }
}
