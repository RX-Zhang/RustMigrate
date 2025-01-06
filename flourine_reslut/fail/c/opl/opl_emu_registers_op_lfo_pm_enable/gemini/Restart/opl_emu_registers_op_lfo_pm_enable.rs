
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: Vec<u8>,
    pub m_waveform: Vec<Vec<u16>>,
}

impl OplEmuRegisters {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: vec![0; 256],
            m_waveform: vec![vec![0; 256]; 4],
        })
    }

    pub fn opl_emu_bitfield(&self, value: u32, start: usize, length: usize) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    pub fn opl_emu_registers_byte(&self, offset: usize, start: usize, count: usize, extra_offset: usize) -> u32 {
        self.opl_emu_bitfield(self.m_regdata[offset + extra_offset] as u32, start, count)
    }

    pub fn opl_emu_registers_op_lfo_pm_enable(&self, opoffs: usize) -> u32 {
        self.opl_emu_registers_byte(0x20, 6, 1, opoffs)
    }
}
