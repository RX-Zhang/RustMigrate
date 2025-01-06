
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let base_offset = (opnum % 18).wrapping_add(2 * ((opnum % 18) / 6));
    base_offset.wrapping_add(0x100u32.wrapping_mul(opnum / 18))
}
