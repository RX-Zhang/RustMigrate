
use std::boxed::Box;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; 0x200],
    pub m_waveform: [[u16; 0x400]; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: u8,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OplEmuFmChannel {
    pub m_choffs: u32,
    pub m_feedback: [i16; 2],
    pub m_feedback_in: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OplEmu {
    pub m_env_counter: u32,
    pub m_status: u8,
    pub m_timer_running: [u8; 2],
    pub m_active_channels: u32,
    pub m_modified_channels: u32,
    pub m_prepare_count: u32,
    pub m_regs: OplEmuRegisters,
    pub m_channel: [OplEmuFmChannel; 18],
    pub m_operator: [OplEmuFmOperator; 36],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub is_op2: i32,
    pub op2_flags: [Op2Flags; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Voicealloc {
    pub priority: u16,
    pub timbreid: i16,
    pub channel: i8,
    pub note: i8,
    pub voiceindex: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Op2Flags {
    pub fixedpitch: u8,
    pub unused: u8,
    pub doublevoice: u8,
}

pub fn opl_loadbank_internal(opl: &mut Opl, file: &str, offset: i32) -> i32 {
    opl.is_op2 = 0;
    let mut buff: [u8; 16] = [0; 16];
    let mut i: i32;
    let mut f = File::open(file).unwrap();
    f.seek(SeekFrom::End(0)).unwrap();
    if f.metadata().unwrap().len() != 3204 {
        return -2;
    }
    f.seek(SeekFrom::Start(0)).unwrap();
    if f.read(&mut buff).unwrap() != 4 || buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
        return -3;
    }
    for i in offset..128 + offset {
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }
        opl.opl_gmtimbres[i as usize].modulator_e862 = (buff[8] as u32) << 8 | (buff[6] as u32) << 16 | (buff[4] as u32) << 24;
        opl.opl_gmtimbres[i as usize].carrier_e862 = (buff[9] as u32) << 8 | (buff[7] as u32) << 16 | (buff[5] as u32) << 24;
        opl.opl_gmtimbres[i as usize].modulator_40 = buff[2];
        opl.opl_gmtimbres[i as usize].carrier_40 = buff[3];
        opl.opl_gmtimbres[i as usize].feedconn = buff[10];
        opl.opl_gmtimbres[i as usize].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i as usize].notenum = 60;
        opl.opl_gmtimbres[i as usize].noteoffset = 0;
    }
    0
}

pub fn opl_loadbank_ibk(opl: &mut Opl, file: &str) -> i32 {
    let mut instruments: String = String::from(file);
    let mut percussion: Option<String> = None;
    let mut i: usize;
    let mut res: i32;
    for i in 0..instruments.len() {
        if instruments.as_bytes()[i] == b',' {
            instruments.truncate(i);
            percussion = Some(instruments[i + 1..].to_string());
            break;
        }
    }
    res = opl_loadbank_internal(opl, &instruments, 0);
    if res == 0 && percussion.is_some() {
        res = opl_loadbank_internal(opl, &percussion.unwrap(), 128);
    }
    res
}
