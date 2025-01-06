

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::ptr::null_mut;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; 18],
    m_operator: [OplEmuFmOperator; 36],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Opl {
    notes2voices: [[[i16; 2]; 128]; 16],
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

fn opl_loadbank_internal(opl: &mut Opl, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut i: i32;

    let mut f = File::open(file).unwrap();
    f.seek(SeekFrom::End(0)).unwrap();
    if f.metadata().unwrap().len() != 3204 {
        return -2;
    }
    f.seek(SeekFrom::Start(0)).unwrap();

    if f.read(&mut buff).unwrap() != 4
        || buff[0] != b'I'
        || buff[1] != b'B'
        || buff[2] != b'K'
        || buff[3] != 0x1A
    {
        return -3;
    }

    for i in offset..128 + offset {
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }

        opl.opl_gmtimbres[i as usize].modulator_e862 = (buff[8] as u32) << 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= buff[6] as u32;
        opl.opl_gmtimbres[i as usize].modulator_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= buff[4] as u32;
        opl.opl_gmtimbres[i as usize].modulator_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].modulator_e862 |= buff[0] as u32;

        opl.opl_gmtimbres[i as usize].carrier_e862 = (buff[9] as u32) << 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= buff[7] as u32;
        opl.opl_gmtimbres[i as usize].carrier_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= buff[5] as u32;
        opl.opl_gmtimbres[i as usize].carrier_e862 <<= 8;
        opl.opl_gmtimbres[i as usize].carrier_e862 |= buff[1] as u32;

        opl.opl_gmtimbres[i as usize].modulator_40 = buff[2];
        opl.opl_gmtimbres[i as usize].carrier_40 = buff[3];

        opl.opl_gmtimbres[i as usize].feedconn = buff[10];
        opl.opl_gmtimbres[i as usize].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i as usize].notenum = 60;
        opl.opl_gmtimbres[i as usize].noteoffset = 0;
    }

    0
}

