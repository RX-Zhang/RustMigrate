
const OPL_EMU_REGISTERS_CHANNELS: usize = 9;
const OPL_EMU_REGISTERS_OPERATORS: usize = 36;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 256;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 3;
const OPL_EMU_REGISTERS_REGISTERS: usize = 256;

#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    OplEmuEgRelease,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

fn opl_emu_fm_channel_init(fmch: &mut OplEmuFmChannel, regs: OplEmuRegisters, choffs: u32) {
    fmch.m_choffs = choffs;
    fmch.m_feedback = [0, 0];
    fmch.m_feedback_in = 0;
    fmch.m_op = [None, None, None, None];
    fmch.m_regs = Box::new(regs);
}

fn opl_emu_fm_operator_init(fmop: &mut OplEmuFmOperator, regs: OplEmuRegisters, opoffs: u32) {
    fmop.m_choffs = 0;
    fmop.m_opoffs = opoffs;
    fmop.m_phase = 0;
    fmop.m_env_attenuation = 0x3ff;
    fmop.m_env_state = OplEmuEnvelopeState::OplEmuEgRelease;
    fmop.m_key_state = 0;
    fmop.m_keyon_live = 0;
    fmop.m_cache = OplEmuOpdataCache {
        phase_step: 0,
        total_level: 0,
        block_freq: 0,
        detune: 0,
        multiple: 0,
        eg_sustain: 0,
        eg_rate: [0; 6],
        eg_shift: 0,
    };
    fmop.m_regs = Box::new(regs);
}
