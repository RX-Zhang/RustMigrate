
pub struct OplEmuRegisters {
    pub m_regdata: Vec<u8>,
}

fn opl_emu_bitfield(value: u32, start: i32, count: i32) -> u32 {
    ((value >> start) % (1 << count)) as u32
}

pub fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[offset.wrapping_add(extra_offset) as usize] as u32,
        start as i32,
        count as i32,
    )
}
