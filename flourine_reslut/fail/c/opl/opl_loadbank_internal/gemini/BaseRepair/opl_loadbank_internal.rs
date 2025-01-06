


use std::boxed::Box;
use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::null;

#[derive(Debug, Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Debug, Clone, Copy)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

#[derive(Debug, Clone, Copy)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; 0x200],
    m_waveform: [[u16; 0x400]; 8],
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

#[derive(Debug, Clone, Copy)]
struct OplTimbre {
    modulator_E862: u32,
    carrier_E862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Debug, Clone, Copy)]
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
    let fsize = f.metadata().unwrap().len();
    if fsize != 3204 {
        return -2;
    }

    if f.read(&mut buff).unwrap() != 4 || buff[0] != b'I' || buff[1] != b'B' || buff[2] != b'K' || buff[3] != 0x1A {
        return -3;
    }

    for i in offset..128 + offset {
        if f.read(&mut buff).unwrap() != 16 {
            return -4;
        }

        opl.opl_gmtimbres[i as usize].modulator_E862 = (buff[8] as u32) << 8 | (buff[6] as u32) << 16 | (buff[4] as u32) << 24;
        opl.opl_gmtimbres[i as usize].carrier_E862 = (buff[9] as u32) << 8 | (buff[7] as u32) << 16 | (buff[5] as u32) << 24;
        opl.opl_gmtimbres[i as usize].modulator_40 = buff[2];
        opl.opl_gmtimbres[i as usize].carrier_40 = buff[3];
        opl.opl_gmtimbres[i as usize].feedconn = buff[10];
        opl.opl_gmtimbres[i as usize].finetune = buff[12] as i8;
        opl.opl_gmtimbres[i as usize].notenum = 60;
        opl.opl_gmtimbres[i as usize].noteoffset = 0;
    }

    0
}


