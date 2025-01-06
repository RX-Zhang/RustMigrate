
use std::boxed::Box;

const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;
const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u32 = 0xff;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;
const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OP2_2NDVOICE_PRIORITY_PENALTY: i32 = 0xFF;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[derive(Clone, Copy)]
enum OplEmuKeyonType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

struct Op2Flags {
    flags: u8,
}

impl Op2Flags {
    const FIXEDPITCH: u8 = 1;
    const UNUSED: u8 = 2;
    const DOUBLEVOICE: u8 = 4;

    fn new() -> Self {
        Op2Flags { flags: 0 }
    }

    fn contains(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }

    fn insert(&mut self, flag: u8) {
        self.flags |= flag;
    }
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}
