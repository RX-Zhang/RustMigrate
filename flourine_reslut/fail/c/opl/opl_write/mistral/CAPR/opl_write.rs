
use std::mem;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Copy, Clone, PartialEq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

#[derive(Copy, Clone, PartialEq)]
enum OplEmuKeyonType {
    OPL_EMU_KEYON_NORMAL = 0,
    OPL_EMU_KEYON_RHYTHM = 1,
    OPL_EMU_KEYON_CSM = 2,
}

#[derive(Copy, Clone)]
struct OplTimbreT {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u8,
    m_operators: Box<[OplOperatorT]>,
}

#[derive(Copy, Clone)]
struct OplOperatorT {
    state: OplEmuEnvelopeState,
    phase: u8,
    shift: u8,
    total_level: u8,
    waveform: u8,
    output_level: u8,
    connection: u8,
    keyon_type: OplEmuKeyonType,
    frequency: u16,
    frequency_f_number: u16,
    frequency_f_number_shift: u8,
    frequency_counter: u16,
    frequency_counter_reload: u16,
    frequency_counter_reload_divider: u8,
    frequency_counter_reload_divider_shift: u8,
    envelope_counter: u8,
    envelope_counter_reload: u8,
    envelope_counter_reload_divider: u8,
    envelope_counter_reload_divider_shift: u8,
    envelope_state: OplEmuEnvelopeState,
    envelope_target: u8,
    envelope_target_shift: u8,
    envelope_target_increment: u8,
    envelope_shaper: u8,
    envelope_shaper_fraction: u8,
    envelope_shaper_increment: u8,
    envelope_shaper_fraction_increment: u8,
    envelope_shaper_shift: u8,
    tremolo_counter: u8,
    tremolo_counter_reload: u8,
    tremolo_counter_reload_divider: u8,
    tremolo_counter_reload_divider_shift: u8,
    tremolo_depth: u8,
    tremolo_waveform: u8,
    vibrato_counter: u8,
    vibrato_counter_reload: u8,
    vibrato_counter_reload_divider: u8,
    vibrato_counter_reload_divider_shift: u8,
    vibrato_depth: u8,
    vibrato_waveform: u8,
    hard_restart: bool,
}
