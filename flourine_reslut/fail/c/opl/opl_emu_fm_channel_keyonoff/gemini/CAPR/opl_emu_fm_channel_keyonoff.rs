
use std::ops::{Add, BitAnd, BitOr, Shr, Shl};

#[derive(Clone, Copy)]
pub enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy)]
pub enum KeyOnType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

#[derive(Clone)]
pub struct Registers {
    pub lfo_am_counter: u16,
    pub lfo_pm_counter: u16,
    pub noise_lfsr: u32,
    pub lfo_am: u8,
    pub regdata: [u8; 0x200],
    pub waveform: [[u16; 0x400]; 8],
}

#[derive(Clone)]
pub struct OpDataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Clone)]
pub struct FmOperator {
    pub choffs: u32,
    pub opoffs: u32,
    pub phase: u32,
    pub env_attenuation: u16,
    pub env_state: EnvelopeState,
    pub key_state: u8,
    pub keyon_live: u8,
    pub cache: OpDataCache,
    pub regs: Box<Registers>,
}

#[derive(Clone)]
pub struct FmChannel {
    pub choffs: u32,
    pub feedback: [i16; 2],
    pub feedback_in: i16,
    pub op: [Option<Box<FmOperator>>; 4],
    pub regs: Box<Registers>,
}

pub fn bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

pub fn fm_operator_keyonoff(fmop: &mut FmOperator, on: u32, type_: KeyOnType) {
    fmop.keyon_live = (fmop.keyon_live & !(1 << type_ as usize)) | ((bitfield(on, 0, 1) as u8) << type_ as usize);
}

pub fn fm_channel_keyonoff(fmch: &mut FmChannel, states: u32, type_: KeyOnType, chnum: u32) {
    for opnum in 0..4 {
        if let Some(op) = &mut fmch.op[opnum] {
            fm_operator_keyonoff(op, bitfield(states, opnum as i32, 1), type_);
        }
    }
}
