
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    (opnum % 18).wrapping_add(2 * ((opnum % 18) / 6))
        .wrapping_add(opnum / 18 * 0x100)
}
