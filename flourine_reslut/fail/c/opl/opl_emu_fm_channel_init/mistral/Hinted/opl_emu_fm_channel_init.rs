

use std::mem;
use std::option::Option;
use std::boxed;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[derive(Copy, Clone)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

#[repr(C)]
struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize], // waveforms
}

struct OplEmuOpdataCache {
    // set phase_step to this value to recalculate it each sample; needed
    // in the case of PM LFO changes

    phase_step: u32,              // phase step, or OPL_EMU_PHASE_STEP_DYNAMIC if PM is active
    total_level: u32,             // total level * 8 + KSL
    block_freq: u32,              // raw block frequency value (used to compute phase_step)
    detune: i32,                   // detuning value (used to compute phase_step)
    multiple: u32,                // multiple value (x.1, used to compute phase_step)
    eg_sustain: u32,              // sustain level, shifted up to envelope values
    eg_rate: [u8; OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize],       // envelope rate, including KSR
    eg_shift: u8,                 // envelope shift amount
}

#[repr(C)]
struct OplEmuFmOperator {
    // internal state
    m_choffs: u32,                     // channel offset in registers
    m_opoffs: u32,                     // operator offset in registers
    m_phase: u32,                      // current phase value (10.10 format)
    m_env_attenuation: u16,            // computed envelope attenuation (4.6 format)
    m_env_state: OplEmuEnvelopeState,            // current envelope state
    m_key_state: u8,                   // current key state: on or off (bit 0)
    m_keyon_live: u8,                  // live key on state (bit 0 = direct, bit 1 = rhythm, bit 2 = CSM)
    m_cache: Box<OplEmuOpdataCache>,                  // cached values for performance
    m_regs: Box<OplEmuRegisters>,                  // direct reference to registers
}

#[repr(C)]
struct OplEmuFmChannel {
    // internal state
    m_choffs: u32,                     // channel offset in registers
    m_feedback: [i16; 2],                 // feedback memory for operator 1
    m_feedback_in: i16,         // next input value for op 1 feedback (set in output)
    m_op: [Option<Box<OplEmuFmOperator>>; 4],    // up to 4 operators
    m_regs: Box<OplEmuRegisters>,                  // direct reference to registers
}

const NONE_OP: Option<Box<OplEmuFmOperator>> = None;

fn opl_emu_fm_channel_init(
    fmch: &mut OplEmuFmChannel,
    regs: Box<OplEmuRegisters>,
    choffs: u32
) {
    *fmch = OplEmuFmChannel {
        m_choffs: choffs,
        m_feedback: [0; 2],
        m_feedback_in: 0,
        m_op: [NONE_OP; 4],
        m_regs: regs,
    };
}

