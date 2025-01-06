
fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    let remainder = chnum.wrapping_rem(9);
    let quotient = chnum.wrapping_div(9);
    remainder.wrapping_add(quotient.wrapping_mul(0x100))
}
