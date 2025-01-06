
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let opnum_mod_18 = opnum % 18;
    let opnum_div_18 = opnum / 18;
    let opnum_mod_18_div_6 = opnum_mod_18 / 6;
    let result = opnum_mod_18
        .wrapping_add(opnum_mod_18_div_6.wrapping_mul(2))
        .wrapping_add(opnum_div_18.wrapping_mul(0x100));
    result
}
