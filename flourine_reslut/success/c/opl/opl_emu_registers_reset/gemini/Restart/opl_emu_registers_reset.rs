
use std::ptr;

#[repr(C)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

impl OplEmuRegisters {
    pub fn reset(&mut self) {
        for i in 0..0x200 {
            unsafe {
                *self.m_regdata.get_unchecked_mut(i) = 0;
            }
        }
    }
}
