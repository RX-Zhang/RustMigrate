
use std::boxed::Box;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct OplEmuFmChannel {
    pub m_choffs: u32,
    pub m_feedback: [i16; 2],
    pub m_feedback_in: i16,
    pub m_op: [Box<OplEmuFmOperator>; 4],
    pub m_regs: Box<OplEmuRegisters>,
}

impl OplEmuFmChannel {
    pub fn new(regs: &mut Box<OplEmuRegisters>, choffs: u32) -> Self {
        Self {
            m_choffs: choffs,
            m_feedback: [0, 0],
            m_feedback_in: 0,
            m_op: [
                Box::new(OplEmuFmOperator {
                    m_choffs: choffs,
                    m_opoffs: 0,
                    m_phase: 0,
                    m_env_attenuation: 0,
                    m_env_state: 0,
                    m_key_state: 0,
                    m_keyon_live: 0,
                    m_cache: OplEmuOpdataCache {
                        phase_step: 0,
                        total_level: 0,
                        block_freq: 0,
                        detune: 0,
                        multiple: 0,
                        eg_sustain: 0,
                        eg_rate: [0; 6],
                        eg_shift: 0,
                    },
                    m_regs: regs.clone(),
                }),
                Box::new(OplEmuFmOperator {
                    m_choffs: choffs,
                    m_opoffs: 1,
                    m_phase: 0,
                    m_env_attenuation: 0,
                    m_env_state: 0,
                    m_key_state: 0,
                    m_keyon_live: 0,
                    m_cache: OplEmuOpdataCache {
                        phase_step: 0,
                        total_level: 0,
                        block_freq: 0,
                        detune: 0,
                        multiple: 0,
                        eg_sustain: 0,
                        eg_rate: [0; 6],
                        eg_shift: 0,
                    },
                    m_regs: regs.clone(),
                }),
                Box::new(OplEmuFmOperator {
                    m_choffs: choffs,
                    m_opoffs: 2,
                    m_phase: 0,
                    m_env_attenuation: 0,
                    m_env_state: 0,
                    m_key_state: 0,
                    m_keyon_live: 0,
                    m_cache: OplEmuOpdataCache {
                        phase_step: 0,
                        total_level: 0,
                        block_freq: 0,
                        detune: 0,
                        multiple: 0,
                        eg_sustain: 0,
                        eg_rate: [0; 6],
                        eg_shift: 0,
                    },
                    m_regs: regs.clone(),
                }),
                Box::new(OplEmuFmOperator {
                    m_choffs: choffs,
                    m_opoffs: 3,
                    m_phase: 0,
                    m_env_attenuation: 0,
                    m_env_state: 0,
                    m_key_state: 0,
                    m_keyon_live: 0,
                    m_cache: OplEmuOpdataCache {
                        phase_step: 0,
                        total_level: 0,
                        block_freq: 0,
                        detune: 0,
                        multiple: 0,
                        eg_sustain: 0,
                        eg_rate: [0; 6],
                        eg_shift: 0,
                    },
                    m_regs: regs.clone(),
                }),
            ],
            m_regs: regs.clone(),
        }
    }
}
