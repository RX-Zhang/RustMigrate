
pub fn rshift32(value: i32, shift: usize) -> i32 {
    let shift = shift % 32;
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding) >> shift) - (value & mask == rounding) as i32) as i32
}
