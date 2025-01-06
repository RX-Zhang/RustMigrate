
pub struct OplEmuRegisters {
    byte_registers: [u8; 256],
}

pub fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32) -> u8 {
    let value = regs.byte_registers[start as usize + offset as usize];
    value
}
