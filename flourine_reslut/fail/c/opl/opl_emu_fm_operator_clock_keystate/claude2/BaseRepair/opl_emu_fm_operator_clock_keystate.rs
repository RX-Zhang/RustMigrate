
use std::boxed::Box;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 256],
    pub m_waveform: [[u16; 512]; 9],
}

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

pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
    pub m_regs: Box<OplEmuRegisters>,
}

pub fn opl_emu_fm_operator_start_release(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state >= OplEmuEnvelopeState::OPL_EMU_EG_RELEASE {
        return;
    }
    fmop.m_env_state = OplEmuEnvelopeState::OPL_EMU_EG_RELEASE;
}

pub fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state == OplEmuEnvelopeState::OPL_EMU_EG_ATTACK {
        return;
    }
    fmop.m_env_state = OplEmuEnvelopeState::OPL_EMU_EG_ATTACK;

    fmop.m_phase = 0;

    if fmop.m_cache.eg_rate[OplEmuEnvelopeState::OPL_EMU_EG_ATTACK as usize] >= 62 {
        fmop.m_env_attenuation = 0;
    }
}

pub fn opl_emu_fm_operator_clock_keystate(fmop: &mut OplEmuFmOperator, keystate: u8) {
    if (keystate as u32 ^ fmop.m_key_state as u32) != 0 {
        fmop.m_key_state = keystate;

        if keystate != 0 {
            opl_emu_fm_operator_start_attack(fmop);
        } else {
            opl_emu_fm_operator_start_release(fmop);
        }
    }
}

