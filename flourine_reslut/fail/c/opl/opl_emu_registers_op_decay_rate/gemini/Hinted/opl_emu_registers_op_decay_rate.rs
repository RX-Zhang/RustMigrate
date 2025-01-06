
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Shl;
use std::ops::Shr;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

pub fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = ((1 << length) - 1) << start;
    (value & mask) >> start
}

pub fn opl_emu_registers_byte(
    regs: &OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.m_regdata[offset as usize + extra_offset as usize] as u32,
        start as i32,
        count as i32,
    )
}

pub fn opl_emu_registers_op_decay_rate(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x60, 0, 4, opoffs)
}
