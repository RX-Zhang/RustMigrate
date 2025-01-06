
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_waveform_select: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_key_scale_level: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_attack_decay: [u8; OPL_EMU_REGISTERS_OPERATORS],
    m_sustain_release: [u8; OPL_EMU_REGISTERS_OPERATORS],
    m_frequency_high: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_frequency_low: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_feedback_connection: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_envelope_state: [OplEmuEnvelopeState; OPL_EMU_REGISTERS_OPERATORS],
    m_envelope_volume: [u8; OPL_EMU_REGISTERS_OPERATORS],
    m_lfo_am_depth: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_lfo_pm_depth_left: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_lfo_pm_depth_right: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_vibrato_depth: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_tremolo_depth: [u8; OPL_EMU_REGISTERS_CHANNELS],
    m_status: u8,
}

fn new_opl_emu_registers() -> OplEmuRegisters {
    OplEmuRegisters {
        m_lfo_am_counter: 0,
        m_lfo_pm_counter: 0,
        m_waveform_select: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_key_scale_level: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_attack_decay: [0; OPL_EMU_REGISTERS_OPERATORS],
        m_sustain_release: [0; OPL_EMU_REGISTERS_OPERATORS],
        m_frequency_high: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_frequency_low: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_feedback_connection: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_envelope_state: [OplEmuEnvelopeState::Release; OPL_EMU_REGISTERS_OPERATORS],
        m_envelope_volume: [0; OPL_EMU_REGISTERS_OPERATORS],
        m_lfo_am_depth: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_lfo_pm_depth_left: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_lfo_pm_depth_right: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_vibrato_depth: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_tremolo_depth: [0; OPL_EMU_REGISTERS_CHANNELS],
        m_status: 0,
    }
}
