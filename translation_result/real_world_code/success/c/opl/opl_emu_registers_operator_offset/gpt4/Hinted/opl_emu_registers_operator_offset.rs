
pub fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let base_offset = 0x100u32.wrapping_mul(opnum / 18);
    let operator_offset = (opnum % 18) + 2u32.wrapping_mul((opnum % 18) / 6);
    
    base_offset.wrapping_add(operator_offset)
}
