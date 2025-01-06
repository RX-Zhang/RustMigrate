
fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = (std::mem::size_of::<i32>() * 8) as i32 - bits as i32;
    let v: i32 = val;
    if shift > 0 {
        let v: i32 = (v << shift) as i32;
        (v >> shift) as i32
    } else {
        val
    }
}
