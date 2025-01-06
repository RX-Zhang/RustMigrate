
struct OplEmuRegisters {
    m_regdata: Box<[u8; 256]>, // Adjust the size as needed
}

fn opl_emu_bitfield(value: u32, start: i32, count: i32) -> u32 {
    let mask = ((1u32.wrapping_shl(count as u32)).wrapping_sub(1)).wrapping_shl(start as u32);
    (value & mask).wrapping_shr(start as u32)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset.wrapping_add(extra_offset)) as usize] as u32, start as i32, count as i32)
}
