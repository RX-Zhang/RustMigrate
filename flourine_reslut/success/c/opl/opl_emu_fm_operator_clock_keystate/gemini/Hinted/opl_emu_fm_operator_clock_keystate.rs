

use std::boxed::Box;

#[repr(C)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

#[repr(C)]
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

#[repr(C)]
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

pub fn opl_emu_fm_operator_start_release(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state >= 4 {
        return;
    }
    fmop.m_env_state = 4;
}

pub fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state == 1 {
        return;
    }
    fmop.m_env_state = 1;
    fmop.m_phase = 0;
    if fmop.m_cache.eg_rate[1] >= 62 {
        fmop.m_env_attenuation = 0;
    }
}

pub fn opl_emu_fm_operator_clock_keystate(fmop: &mut OplEmuFmOperator, keystate: u8) {
    if fmop.m_key_state != keystate {
        fmop.m_key_state = keystate;
        if keystate != 0 {
            opl_emu_fm_operator_start_attack(fmop);
        } else {
            opl_emu_fm_operator_start_release(fmop);
        }
    }
}


