
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Box<[u8; 512]>,
    pub m_waveform: Box<[[u16; 1024]; 8]>,
}

impl OplEmuRegisters {
    pub fn op_attack_rate(&self, opoffs: u32) -> u32 {
        OplEmuRegisters::byte(self, 0x60, 4, 4, opoffs)
    }

    fn bitfield(value: u32, start: u32, length: u32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    fn byte(&self, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
        OplEmuRegisters::bitfield(self.m_regdata[offset as usize + extra_offset as usize] as u32, start, count)
    }
}
