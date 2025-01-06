
use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

#[derive(Debug)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Debug)]
pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: u8,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
    pub m_regs: Box<OplEmuRegisters>,
}

impl OplEmuFmOperator {
    pub fn start_release(&mut self) {
        if self.m_env_state >= 4 {
            return;
        }
        self.m_env_state = 4;
    }

    pub fn start_attack(&mut self) {
        if self.m_env_state == 1 {
            return;
        }
        self.m_env_state = 1;
        self.m_phase = 0;
        if self.m_cache.eg_rate[0] >= 62 {
            self.m_env_attenuation = 0;
        }
    }

    pub fn clock_keystate(&mut self, keystate: u32) {
        if (keystate & 0xff) != self.m_key_state as u32 {
            self.m_key_state = (keystate & 0xff) as u8;
            if self.m_key_state != 0 {
                self.start_attack();
            } else {
                self.start_release();
            }
        }
    }
}
