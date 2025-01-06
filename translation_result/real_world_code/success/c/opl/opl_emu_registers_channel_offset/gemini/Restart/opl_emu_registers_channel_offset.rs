
fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    let offset = chnum % 9;
    let base = chnum / 9;
    (base as u32).wrapping_mul(0x100).wrapping_add(offset)
}
