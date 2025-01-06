
use std::boxed::Box;

#[derive(Clone, Copy, Debug)]
pub enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy, Debug)]
pub enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

#[derive(Clone, Copy, Debug)]
pub struct OplEmuRegisters {
    pub lfo_am_counter: u16,
    pub lfo_pm_counter: u16,
    pub noise_lfsr: u32,
    pub lfo_am: u8,
    pub regdata: [u8; 0x200],
    pub waveform: [[u16; 0x400]; 8],
}

#[derive(Clone, Copy, Debug)]
pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct OplEmuFmOperator {
    pub choffs: u32,
    pub opoffs: u32,
    pub phase: u32,
    pub env_attenuation: u16,
    pub env_state: OplEmuEnvelopeState,
    pub key_state: u8,
    pub keyon_live: u8,
    pub cache: OplEmuOpdataCache,
}

#[derive(Clone, Copy, Debug)]
pub struct OplEmuFmChannel {
    pub choffs: u32,
    pub feedback: [i16; 2],
    pub feedback_in: i16,
}

#[derive(Clone, Copy, Debug)]
pub struct OplTimbre {
    pub modulator_e862: u32,
    pub carrier_e862: u32,
    pub modulator_40: u8,
    pub carrier_40: u8,
    pub feedconn: u8,
    pub finetune: i8,
    pub notenum: u8,
    pub noteoffset: i16,
}

#[derive(Clone, Copy, Debug)]
pub struct OplEmu {
    pub env_counter: u32,
    pub status: u8,
    pub timer_running: [u8; 2],
    pub active_channels: u32,
    pub modified_channels: u32,
    pub prepare_count: u32,
    pub regs: OplEmuRegisters,
    pub channel: [OplEmuFmChannel; 18],
    pub operator: [OplEmuFmOperator; 36],
}

#[derive(Clone, Copy, Debug)]
pub struct Voicealloc {
    pub priority: u16,
    pub timbreid: i16,
    pub channel: i8,
    pub note: i8,
    pub voiceindex: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Opl {
    pub notes2voices: [[[i8; 2]; 128]; 16],
    pub channelpitch: [u16; 16],
    pub channelvol: [u16; 16],
    pub voices2notes: [Voicealloc; 18],
    pub channelprog: [u8; 16],
    pub opl3: i32,
    pub opl_emu: OplEmu,
    pub opl_gmtimbres: [OplTimbre; 256],
    pub opl_gmtimbres_voice2: [OplTimbre; 256],
    pub is_op2: bool,
    pub op2_flags: [Op2Flags; 256],
}

pub fn opl_midi_changeprog(opl: &mut Opl, channel: i32, program: i32) {
    if channel == 9 {
        return;
    }
    opl.channelprog[channel as usize] = program as u8;
}
