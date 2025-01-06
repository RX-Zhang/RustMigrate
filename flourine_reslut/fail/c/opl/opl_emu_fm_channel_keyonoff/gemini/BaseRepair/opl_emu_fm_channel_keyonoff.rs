
use std::ops::{Add, Shl, Shr};

#[derive(Copy, Clone)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Copy, Clone)]
enum OplEmuKeyonType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
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

struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
    regs: Box<OplEmuRegisters>,
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
    op: [Option<Box<OplEmuFmOperator>>; 4],
    regs: Box<OplEmuRegisters>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(
    fmop: &mut OplEmuFmOperator,
    on: u32,
    type_: OplEmuKeyonType,
) {
    fmop.keyon_live = (fmop.keyon_live & !(1 << type_ as usize))
        | (((on & 1) as u8) << type_ as usize);
}

fn opl_emu_fm_channel_keyonoff(
    fmch: &mut OplEmuFmChannel,
    states: u32,
    type_: OplEmuKeyonType,
    chnum: u32,
) {
    for opnum in 0..fmch.op.len() {
        if let Some(fmop) = &mut fmch.op[opnum] {
            opl_emu_fm_operator_keyonoff(fmop, opl_emu_bitfield(states, opnum as i32, 1), type_);
        }
    }
}
