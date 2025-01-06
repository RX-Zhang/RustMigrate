

use std::mem;
use std::usize;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op2Flags {
    OP2_FIXEDPITCH = 1,
    OP2_UNUSED = 2,
    OP2_DOUBLEVOICE = 4,
}

struct OplTimbre;
struct VoiceAlloc;

struct Opl {
    notes2voices: [[[usize; 2]; 128]; 16],
    channelpitch: [usize; 16],
    channelvol: [usize; 16],
    voices2notes: Box<[VoiceAlloc; 18]>,
    channelprog: [usize; 16],
    opl3: bool,
    opl_emu: OplEmuRegisters,
    opl_gmtimbres: Box<[OplTimbre; 256]>,
    opl_gmtimbres_voice2: Box<[OplTimbre; 256]>,
    is_op2: bool,
    op2_flags: Box<[Op2Flags; 256]>,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u8,
}

