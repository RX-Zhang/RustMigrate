
use std::boxed::Box;

pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Box<[u8; 0x200]>,
    pub m_waveform: Box<[[u16; 0x400]; 8]>,
}

pub fn opl_emu_registers_lfo_am_offset(regs: &mut OplEmuRegisters, choffs: u32) -> u32 {
    regs.m_lfo_am as u32
}
