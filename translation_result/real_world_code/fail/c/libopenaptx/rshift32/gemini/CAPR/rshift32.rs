
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift % 32 - 1);
    let mask = (1 << (shift % 32 + 1)) - 1;
    ((value.wrapping_add(rounding)) >> (shift % 32)) - ((value & mask) == rounding as i32) as i32
}
