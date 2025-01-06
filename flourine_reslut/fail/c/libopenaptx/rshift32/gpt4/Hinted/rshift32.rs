
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32.wrapping_shl(shift - 1);
    let mask = (1i32.wrapping_shl(shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i32
}

fn main() {
    println!("{}", rshift32(-1129553664, 14)); // -68942
    println!("{}", rshift32(-55811840, 14)); // -3406
    println!("{}", rshift32(-519438324, 4008575214)); // -31704
    println!("{}", rshift32(65535, 14302976)); // -2147418113
}
