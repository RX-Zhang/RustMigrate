
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy, Debug)]
struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
    regs: *const OplEmuRegisters,
}

#[derive(Clone, Copy, Debug)]
struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
    op: [*const OplEmuFmOperator; 4],
    regs: *const OplEmuRegisters,
}

#[derive(Clone, Copy, Debug)]
struct OplEmu {
    env_counter: u32,
    status: u8,
    timer_running: [u8; 2],
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: [OplEmuFmChannel; 18],
    operator: [OplEmuFmOperator; 36],
}

#[derive(Clone, Copy, Debug)]
struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Clone, Copy, Debug)]
struct Opl {
    notes2voices: [[[i8; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [Voicealloc; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: i32,
    op2_flags: [Op2Flags; 256],
}

#[derive(Clone, Copy, Debug)]
struct OplTimbre {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

#[derive(Clone, Copy, Debug)]
enum Op2Flags {
    Fixedpitch = 1,
    Unused = 2,
    Doublevoice = 4,
}

impl Add for OplEmuOpdataCache {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            phase_step: self.phase_step.wrapping_add(other.phase_step),
            total_level: self.total_level.wrapping_add(other.total_level),
            block_freq: self.block_freq.wrapping_add(other.block_freq),
            detune: self.detune.wrapping_add(other.detune),
            multiple: self.multiple.wrapping_add(other.multiple),
            eg_sustain: self.eg_sustain.wrapping_add(other.eg_sustain),
            eg_rate: [
                self.eg_rate[0].wrapping_add(other.eg_rate[0]),
                self.eg_rate[1].wrapping_add(other.eg_rate[1]),
                self.eg_rate[2].wrapping_add(other.eg_rate[2]),
                self.eg_rate[3].wrapping_add(other.eg_rate[3]),
                self.eg_rate[4].wrapping_add(other.eg_rate[4]),
                self.eg_rate[5].wrapping_add(other.eg_rate[5]),
            ],
            eg_shift: self.eg_shift.wrapping_add(other.eg_shift),
        }
    }
}

impl AddAssign for OplEmuOpdataCache {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn getinstrument(opl: &mut Opl, channel: i32, note: i32) -> i32 {
    if note < 0 || note > 127 || channel > 15 {
        return -1;
    }
    if channel == 9 {
        if opl.is_op2 != 0 {
            return 128 + note - 35;
        } else {
            return 128 | note;
        }
    }
    opl.channelprog[channel as usize] as i32
}
