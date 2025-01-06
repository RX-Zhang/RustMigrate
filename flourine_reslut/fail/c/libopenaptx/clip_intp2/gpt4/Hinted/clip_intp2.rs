
fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p)) & !((2u32 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn main() {
    println!("{}", clip_intp2(285343762, 0)); // Expected output: 0
    println!("{}", clip_intp2(134409474, 0)); // Expected output: 0
    println!("{}", clip_intp2(134272522, 389889282)); // Expected output: 3
    println!("{}", clip_intp2(-1, 201326591)); // Expected output: -1
}
