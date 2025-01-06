
pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let start = start.rem_euclid(32);
    let length = length.rem_euclid(32);
    (value.wrapping_shr(start as u32)) & ((1u32 << length as u32) - 1)
}
