

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::States as usize],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u32,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,
}

impl OplEmuFmOperator {
    fn new(regs: Box<OplEmuRegisters>) -> Self {
        OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: 0,
            m_phase: 0,
            m_env_attenuation: 0x3ff,
            m_env_state: OplEmuEnvelopeState::Release,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: [0; OplEmuEnvelopeState::States as usize],
                eg_shift: 0,
            },
            m_regs: regs,
        }
    }

    fn reset(&mut self) {
        self.m_phase = 0;
        self.m_env_attenuation = 0x3ff;
        self.m_env_state = OplEmuEnvelopeState::Release;
        self.m_key_state = 0;
        self.m_keyon_live = 0;
    }
}
