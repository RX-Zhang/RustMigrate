
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let opnum_mod_18 = opnum % 18;
    let base_offset = opnum_mod_18.wrapping_add(2 * (opnum_mod_18 / 6));
    let extended_offset = (opnum / 18).wrapping_mul(0x100);
    base_offset.wrapping_add(extended_offset)
}
