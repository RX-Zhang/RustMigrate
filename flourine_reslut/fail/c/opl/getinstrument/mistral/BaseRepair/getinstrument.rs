

use std::mem;

const OPL_REGISTERS_CHANNELS: usize = 18;
const OPL_REGISTERS_WAVEFORMS: usize = 8;
const OPL_REGISTERS_REGISTERS: usize = 0x200;
const OPL_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
Attack = 1,
Decay = 2,
Sustain = 3,
Release = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op2Flags {
FixedPitch = 1,
UNUSED = 2,
DoubleVoice = 4,
}

struct Opl {
notes2voices: [[[Option<usize>; 2]; 128]; 16],
channelpitch: [u16; 16],
channelvol: [u16; 16],
voices2notes: [Option<(usize, i16, i16)>; 18],
channelprog: [u8; 16],
opl3: bool,
opl_emu: OplEmuRegisters,
opl_gmtimbres: Box<[OplTimbre; 256]>,
opl_gmtimbres_voice2: Box<[OplTimbre; 256]>,
is_op2: bool,
op2_flags: Box<[Op2Flags; 256]>,
}

struct OplEmuRegisters {
m_lfo_am_counter: u16,
m_lfo_pm_counter: u16,
m_noise_lfsr: u32,
m_lfo_am: u8,
m_regdata: [u8; OPL_REGISTERS_REGISTERS],
m_waveform: [[u16; OPL_REGISTERS_WAVEFORM_LENGTH]; OPL_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
phase_step: u32,
}

#[derive(Clone, Copy)]
struct OplTimbre {
freq: u32,
wave: u8,
feedback: u8,
eg_level: u8,
eg_inc: u8,
ksr: u8,
mul: u8,
tl: u8,
}

const OPL_REGISTERS_OPERATORS: usize = OPL_REGISTERS_CHANNELS * 2;

