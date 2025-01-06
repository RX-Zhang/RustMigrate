
use libc::{c_int, c_uint};

const LEN: usize = 10;

fn main() {
    let mut a: [c_uint; LEN] = [0; LEN];
    let mut b: [c_uint; LEN] = [0; LEN];
    let c: &mut [c_uint; LEN] = &mut a;
    let d: &[c_uint; LEN] = &b;

    for i in 0..LEN {
        c[i] = c_uint::wrapping_add(c[i], d[i]);
    }
}
