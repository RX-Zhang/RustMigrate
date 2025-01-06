
fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p)) & !((2u32 << p) - 1) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

fn main() {
    println!("{}", clip_intp2(264, 3)); // Expected output: 33
    println!("{}", rshift32(-1073111040, 5)); // Expected output: -33554432 (if directly calling rshift32)
    println!("{}", rshift32_clip24(-1073111040, 5)); // Expected output: -8388608
    println!("{}", clip_intp2(524287, 1543438592)); // Expected output: -8388608
    println!("{}", clip_intp2(1476394816, 798162943)); // Expected output: -1
}
