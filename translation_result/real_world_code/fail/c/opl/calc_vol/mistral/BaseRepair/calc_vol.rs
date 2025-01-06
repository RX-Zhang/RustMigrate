
fn rotate_left(val: i32, shift_amt: u8) -> i32 {
let shifted_bits = (val as u32).wrapping_shl(shift_amt as u32);
let mask = !((!0u32) << (8 * std::mem::size_of::<u8>() as u32 - shift_amt as u32));
(shifted_bits & mask) as i32
}

fn rotate_right(val: i32, shift_amt: u8) -> i32 {
let shifted_bits = (val as u32).wrapping_shr(shift_amt as u32);
let mask = !((!0u32) << (8 * std::mem::size_of::<u8>() as u32 - shift_amt as u32));
(shifted_bits & mask) as i32
}
