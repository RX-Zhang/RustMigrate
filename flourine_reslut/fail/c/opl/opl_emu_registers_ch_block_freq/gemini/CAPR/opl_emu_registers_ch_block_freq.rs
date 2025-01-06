
use std::boxed::Box;

pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

pub fn opl_emu_bitfield(start: i32, length: i32, value: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

pub fn opl_emu_registers_byte(
    regs: &Box<OplEmuRegisters>,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(start as i32, count as i32, regs.m_regdata[(offset + extra_offset) as usize] as u32)
}

pub fn opl_emu_registers_word(
    regs: &Box<OplEmuRegisters>,
    offset1: u32,
    start1: u32,
    count1: u32,
    offset2: u32,
    start2: u32,
    count2: u32,
    extra_offset: u32,
) -> u32 {
    (opl_emu_registers_byte(regs, offset1, start1, count1, extra_offset) << count2)
        | opl_emu_registers_byte(regs, offset2, start2, count2, extra_offset)
}

pub fn opl_emu_registers_ch_block_freq(regs: &Box<OplEmuRegisters>, choffs: u32) -> u32 {
    opl_emu_registers_word(regs, 0xb0, 0, 5, 0xa0, 0, 8, choffs)
}
