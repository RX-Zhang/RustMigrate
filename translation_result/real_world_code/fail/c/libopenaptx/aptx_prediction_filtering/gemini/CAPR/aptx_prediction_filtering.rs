
fn clip_intp2(a: i32, p: u32) -> i32 {
    let p_shift = 1 << p;
    let p_mask = (2 << p) - 1;

    if ((a as u32) + p_shift) & !p_mask != 0 {
        if a < 0 {
            (p_shift - 1) as i32
        } else {
            (!((p_shift - 1) as u32)).wrapping_add(1) as i32
        }
    } else {
        a
    }
}
