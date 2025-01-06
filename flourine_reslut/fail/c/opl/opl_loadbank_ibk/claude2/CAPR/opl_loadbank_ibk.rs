

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::boxed::Box;
use std::vec::Vec;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;

const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,  
}

enum Op2Flags {
    Op2Fixedpitch = 1,
    Op2Unused = 2,
    Op2Doublevoice = 4
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::OplEmuEgStates as usize],
    eg_shift: u8
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: Box<OplEmuOpdataCache>
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16    
}

struct OplTimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16
}

struct OplEmu;

struct Opl {
    notes2voices: [[[[i8; 2]; 128]; 16]; 2],
    channelpitch: [u16; 16],   
    channelvol: [u16; 16],
    voices2notes: [Box<VoiceAlloc>; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: Box<OplEmu>,
    opl_gmtimbres: [Box<OplTimbre>; 256],
    opl_gmtimbres_voice2: [Box<OplTimbre>; 256],  
    is_op2: bool,
    op2_flags: [Op2Flags; 256]
}

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8  
}

fn opl_loadbank_internal(opl: &mut Opl, file: &str, offset: i32) -> i32 {
    offset
}

fn opl_loadbank_ibk(opl: &mut Opl, file: &str) -> i32 {
    0 // return 0
}




