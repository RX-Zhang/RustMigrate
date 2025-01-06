

use std::mem;
use std::option::Option;
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u32 = 6;

struct OplEmuFmOperator;

struct OplEmuFmChannel {
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
}

fn opl_emu_fm_channel_is4op(fmch: &OplEmuFmChannel) -> bool {
    fmch.m_op[2].is_some()
}

