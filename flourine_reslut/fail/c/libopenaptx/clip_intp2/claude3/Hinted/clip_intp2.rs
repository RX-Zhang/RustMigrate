
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a = a as u32;
    let p = p;
    let max_val = (1 << p) - 1;
    let min_val = !(max_val as i32);

    if a.wrapping_add(1 << p) & !(((2 << p) - 1)) != 0 {
        if a >> 31 != 0 {
            min_val
        } else {
            max_val as i32
        }
    } else {
        a as i32
    }
}
