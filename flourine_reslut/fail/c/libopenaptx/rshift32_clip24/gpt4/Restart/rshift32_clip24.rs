
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (((a as u32).wrapping_add(1u32 << p)) & !(((2u32 << p) - 1))) != 0 {
        return ((a >> 31) ^ ((1 << p) - 1)) as i32;
    } else {
        return a;
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1i32 << (shift - 1);
    let mask: i32 = ((1i32 << (shift + 1)) - 1);
    return (((value + rounding) >> shift) - ((value & mask) == rounding) as i32) as i32;
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    return clip_intp2(rshift32(value, shift), 23);
}

fn main() {
    // Example usage
    let result = rshift32_clip24(123456, 2);
    println!("Result: {}", result);
}
