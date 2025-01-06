
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

#[derive(Debug)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: Vec<u8>,
    pub eg_shift: u8,
}

#[derive(Debug)]
pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: Box<OplEmuOpdataCache>,
    pub m_regs: Box<OplEmuRegisters>,
}

#[derive(Debug)]
pub enum OplEmuEnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl OplEmuFmOperator {
    pub fn new(regs: Box<OplEmuRegisters>, opoffs: u32) -> Self {
        let mut fmop = OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: opoffs,
            m_phase: 0,
            m_env_attenuation: 0x3ff,
            m_env_state: OplEmuEnvelopeState::Release,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: Box::new(OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: vec![0; 6],
                eg_shift: 0,
            }),
            m_regs: regs,
        };

        fmop
    }
}
