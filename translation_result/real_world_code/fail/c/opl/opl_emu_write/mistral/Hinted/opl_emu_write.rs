
use std::mem;

const OPL_U_REGISTERS_OPERATORS: usize = 36;
const OPLEMU_REGISTERS_ALL_CHANNELS: u32 = ((1 << 18) - 1);
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: usize = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Copy, Clone, Eq, PartialEq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
   PL_EMU_EG_AY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u32 = 6;

#[derive(Copy, Clone, Eq, PartialEq)]
enum OplEmuKeyonType {
    OPL_EMU_KEYON_NORMAL = 0,
    OPL_EMU_KEYON_RHYTHM = 1,
    OPL_EMU_KEYON_CSM = 2,
}

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

#[repr(C)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES as usize],
    eg_shift: u8,
}

#[repr(C)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: &'static mut OplEmuRegisters,
}

#[repr(C)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [&'static mut OplEmuFmOperator; 4],
}

#[repr(C)]
struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_U_REGISTERS_OPERATORS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, typ: OplEmuKeyonType) {
    if on == 0 {
        fmop.m_key_state = 0;
        fmop.m_env_state = OplEmuEnvelopeState::OPL_EMU_EG_RELEASE;
        fmop.m_env_attenuation = 0;
    } else {
        fmop.m_key_state = 1;
        fmop.m_env_attenuation = 0x3fff;
        fmop.m_env_state = match typ {
            OplEmuKeyonType::OPL_EMU_KEYON_NORMAL => OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            OplEmuKeyonType::OPL_EMU_KEYON_RHYTHM => OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            OplEmuKeyonType::OPL_EMU_KEYON_CSM => OplEmuEnvelopeState::OPL_EMU_EG_SUSTAIN,
        };
    }
}
