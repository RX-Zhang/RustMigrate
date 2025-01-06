
fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    let modulo = chnum.wrapping_rem(9);
    let division = chnum.wrapping_div(9);
    modulo.wrapping_add(0x100u32.wrapping_mul(division))
}
