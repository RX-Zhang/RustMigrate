
fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p) & !((2u32 << p).wrapping_sub(1))) != 0 {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding) >> shift) - ((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn main() {
    // Example 0
    println!("{}", rshift64_clip24(217020984693033482, 1));
    // Example 1
    println!("{}", rshift64_clip24(263882382248574, 1));
    // Example 2
    println!("{}", rshift64_clip24(-1008806316528202355, 1090371582));
    // Example 3
    println!("{}", rshift64_clip24(8029758751228202387, 1878916975));
}
