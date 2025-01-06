
pub fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value.wrapping_shr(start)) & ((1u32.wrapping_shl(length)) - 1)
}
