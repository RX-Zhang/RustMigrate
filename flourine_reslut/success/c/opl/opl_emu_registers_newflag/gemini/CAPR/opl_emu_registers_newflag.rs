
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16, // LFO AM counter
    m_lfo_pm_counter: u16, // LFO PM counter
    m_noise_lfsr: u32,     // noise LFSR state
    m_lfo_am: u8,          // current LFO AM value
    m_regdata: Box<[u8; 0x200]>, // register data
    m_waveform: Box<[[u16; 0x400]; 8]>, // waveforms
}

impl OplEmuRegisters {
    pub fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
        (value >> start) & ((1 << length) - 1)
    }

    pub fn opl_emu_registers_byte(
        &self,
        offset: u32,
        start: u32,
        count: u32,
        extra_offset: u32,
    ) -> u32 {
        self.opl_emu_bitfield(
            self.m_regdata[offset as usize + extra_offset as usize] as u32,
            start as i32,
            count as i32,
        )
    }

    pub fn opl_emu_registers_newflag(&self) -> u32 {
        self.opl_emu_registers_byte(0x105, 0, 1, 0)
    }
}
