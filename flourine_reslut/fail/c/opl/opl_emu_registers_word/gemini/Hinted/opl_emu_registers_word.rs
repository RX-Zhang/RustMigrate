
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

impl OplEmuRegisters {
    pub fn opl_emu_bitfield(&self, value: u32, start: usize, length: usize) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    pub fn opl_emu_registers_byte(
        &self,
        offset: usize,
        start: usize,
        count: usize,
        extra_offset: usize,
    ) -> u32 {
        self.opl_emu_bitfield(self.m_regdata[offset + extra_offset] as u32, start, count)
    }

    pub fn opl_emu_registers_word(
        &self,
        offset1: usize,
        start1: usize,
        count1: usize,
        offset2: usize,
        start2: usize,
        count2: usize,
        extra_offset: usize,
    ) -> u32 {
        (self.opl_emu_registers_byte(offset1, start1, count1, extra_offset) << count2)
            | self.opl_emu_registers_byte(offset2, start2, count2, extra_offset)
    }
}
