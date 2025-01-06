
use std::ops::Shr;
use std::mem;

// Function signature is the same as the function
pub fn rotl(x: i32, k: i32) -> i32 {
    // Extract the sign bit of k
    let k_sign_bit = k.shr(31) & 1;
    
    // Calculate the bit length of x as a i32
    let x_bit_length: i32 = mem::size_of::<i32>() as i32 * 8;
    
    // Calculate the rotate amount as (k % x_bit_length)
    let rotate_amount = (k + k_sign_bit * x_bit_length) % x_bit_length as i32;
    
    // Shift x to the left by rotate_amount
    let shifted_left = x << rotate_amount;
    
    // Calculate the number of bits to shift to the right
    let shift_right_amount = x_bit_length - rotate_amount;
    
    // Shift x to the right by shift_right_amount
    let shifted_right = x.wrapping_shr(shift_right_amount as u32);
    
    // Combine the shifted left and shifted right values
    shifted_left | shifted_right
}
