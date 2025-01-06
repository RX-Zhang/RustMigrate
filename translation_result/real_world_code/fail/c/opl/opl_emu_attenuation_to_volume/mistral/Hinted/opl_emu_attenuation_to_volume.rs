
fn c_shift_left(val: i32, shift_amount: u32) -> i32 {
let mut shifted_val = val as u32;
shifted_val = shifted_val.wrapping_shl(shift_amount);
shifted_val as i32
}

fn c_shift_right(val: i32, shift_amount: u32) -> i32 {
let mut shifted_val = val as u32;
shifted_val = shifted_val.wrapping_shr(shift_amount);
shifted_val as i32
}
