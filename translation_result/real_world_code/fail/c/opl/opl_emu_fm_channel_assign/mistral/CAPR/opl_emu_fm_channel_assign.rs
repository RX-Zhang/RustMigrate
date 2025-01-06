
fn rotate_left(value: u32, shift_amount: u8) -> u32 {
value.wrapping_shl((8 * (shift_amount as u32).saturating_sub(std::mem::size_of::<u32>() as u32 * 8)) % (std::mem::size_of::<u32>() as u32 * 8) as u32) |
value.wrapping_shr((std::mem::size_of::<u32>() as u32 * 8 - shift_amount as u32) % (std::mem::size_of::<u32>() as u32 * 8) as u32)
}
