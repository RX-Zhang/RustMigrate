

use std::fs::File;
use std::io::{Read, Result};
use std::mem;
use std::ops::{BitAnd, BitOr, Shl, Shr};
use std::path::Path;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_EG_STATES: usize = 6;

#[derive(Copy, Clone, PartialEq)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Copy, Clone)]
enum Op2FlagsT {
    Op2Fixedpitch = 1,
    Op2Unused = 2,
    Op2Doublevoice = 4,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: Box<[u8; OPL_EMU_EG_STATES]>,
    eg_shift: u8,
}

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

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: Box<[i16; 2]>,
    m_feedback_in: i16,
}

struct OplTimbreT {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: Box<[u8; 2]>,
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: Box<[OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS]>,
    m_operator: Box<[OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS]>,
}

struct VoiceallocT {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct OplT {
    notes2voices: Box<[[[i8; 2]; 128]; 16]>,
    channelpitch: Box<[u16; 16]>,
    channelvol: Box<[u16; 16]>,
    voices2notes: Box<[VoiceallocT; 18]>,
    channelprog: Box<[u8; 16]>,
    opl3: i8,
    opl_emu: OplEmuT,
    opl_gmtimbres: Box<[OplTimbreT; 256]>,
    opl_gmtimbres_voice2: Box<[OplTimbreT; 256]>,
    is_op2: i32,
    op2_flags: Box<[Op2FlagsT; 256]>,
}

fn opl_loadbank_internal(opl: &mut OplT, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff = [0u8; 16];
    let mut i: i32;

    let mut f = match File::open(Path::new(file)) {
        Ok(file) => file,
        Err(_) => return -1,
    };

    if f.metadata().unwrap().len() != 3204 {
        return -2;
    }

    if f.read(&mut buff).unwrap() != 4
        || buff[0] != b'I'
        || buff[1] != b'B'
        || buff[2] != b'K'
        || buff[3] != 0x1A
    {
        return -3;
    }

    for i in offset..offset + 128 {
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }

        opl.opl_gmtimbres[(i as usize)].modulator_E862 = (buff[8] as u32) << 24;
        opl.opl_gmtimbres[(i as usize)].modulator_E862 |= (buff[6] as u32) << 16;
        opl.opl_gmtimbres[(i as usize)].modulator_E862 |= (buff[4] as u32) << 8;
        opl.opl_gmtimbres[(i as usize)].modulator_E862 |= buff[0] as u32;

        opl.opl_gmtimbres[(i as usize)].carrier_E862 = (buff[9] as u32) << 24;
        opl.opl_gmtimbres[(i as usize)].carrier_E862 |= (buff[7] as u32) << 16;
        opl.opl_gmtimbres[(i as usize)].carrier_E862 |= (buff[5] as u32) << 8;
        opl.opl_gmtimbres[(i as usize)].carrier_E862 |= buff[1] as u32;

        opl.opl_gmtimbres[(i as usize)].modulator_40 = buff[2];
        opl.opl_gmtimbres[(i as usize)].carrier_40 = buff[3];

        opl.opl_gmtimbres[(i as usize)].feedconn = buff[10];
        opl.opl_gmtimbres[(i as usize)].finetune = buff[12] as i8;
        opl.opl_gmtimbres[(i as usize)].notenum = 60;
        opl.opl_gmtimbres[(i as usize)].noteoffset = 0;
    }

    0
}

fn opl_loadbank_ibk(opl: &mut OplT, file: &str) -> i32 {
    let mut instruments: Option<String> = None;
    let mut percussion: Option<String> = None;
    let mut i: usize = 0;
    let mut res: i32;

    instruments = Some(file.to_owned());

    if let Some(instruments) = instruments.as_mut() {
        let mut bytes = instruments.as_bytes().to_vec();
        while i < bytes.len() {
            if bytes[i] == b',' {
                bytes[i] = 0;
                percussion = Some(String::from_utf8(bytes.split_off(i + 1)).unwrap());
                break;
            }
            i += 1;
        }
        *instruments = String::from_utf8(bytes).unwrap();
    }

    res = opl_loadbank_internal(opl, &instruments.unwrap(), 0);
    if res == 0 && percussion.is_some() {
        res = opl_loadbank_internal(opl, &percussion.unwrap(), 128);
    }

    res
}

