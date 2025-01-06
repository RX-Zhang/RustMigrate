
use std::convert::TryInto;

pub struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16, 
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

pub struct OplEmuRegistersOperatorMapping {
    chan: [u32; 18], 
}

pub struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,  
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

pub struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,  
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: EnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,  
}

pub struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,
}

pub struct OplEmu {
    m_env_counter: u32,
    m_status: u8, 
    m_timer_running: [u8; 2],  
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; 18],
    m_operator: [OplEmuFmOperator; 36],
}

#[repr(u8)]
enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

// Rest of functions

