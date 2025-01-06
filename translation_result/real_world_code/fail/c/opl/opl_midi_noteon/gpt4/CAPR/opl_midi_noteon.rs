
fn opl_emu_bitfield(value: u32, offset: u32, width: u32) -> u32 {
    (value.wrapping_shr(offset % 32)) & ((1u32.wrapping_shl(width % 32)).wrapping_sub(1))
}
