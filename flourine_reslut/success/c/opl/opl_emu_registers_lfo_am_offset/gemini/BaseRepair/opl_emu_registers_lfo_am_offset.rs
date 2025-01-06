
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    // internal state
    pub m_lfo_am_counter: u16,            // LFO AM counter
    pub m_lfo_pm_counter: u16,            // LFO PM counter
    pub m_noise_lfsr: u32,                // noise LFSR state
    pub m_lfo_am: u8,                     // current LFO AM value
    pub m_regdata: Box<[u8; 0x200]>,         // register data
    pub m_waveform: Box<[[u16; 0x400]; 8]>, // waveforms
}

impl OplEmuRegisters {
    pub fn lfo_am_offset(&self, choffs: u32) -> u32 {
        self.m_lfo_am as u32
    }
}
