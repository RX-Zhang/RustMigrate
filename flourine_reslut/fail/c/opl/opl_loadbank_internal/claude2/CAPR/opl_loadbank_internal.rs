

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::boxed::Box;

struct OplEmu;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;

const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

enum Op2Flags {
    OP2_FIXEDPITCH = 1,
    OP2_UNUSED = 2, 
    OP2_DOUBLEVOICE = 4,
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
    eg_rate: [u8; OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize],  
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
    m_cache: Box<OplEmuOpdataCache>,
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
    op2_flags: [Op2Flags; 256],
}

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8, 
}

fn opl_loadbank_internal(opl: &mut Opl, file: &str, offset: i32) -> i32 {
    opl.is_op2 = false;

    let mut buff = [0u8; 16];
    let mut f = match File::open(file) {
        Ok(f) => f,
        Err(_) => return -1,
    };

    f.seek(SeekFrom::End(0)).unwrap();
    if f.stream_position().unwrap() != 3204 {
        return -2;
    }

    f.seek(SeekFrom::Start(0)).unwrap();
    f.read(&mut buff).unwrap();

    if buff[0..4] != *b"IBK\x1A" {
        return -3;
    }

    for i in offset..(offset + 128) {
        f.read(&mut buff).unwrap();

        opl.opl_gmtimbres[i as usize] = Box::new(OplTimbre {
            modulator_E862: u32::from_le_bytes([buff[0], buff[4], buff[6], buff[8]]),
            carrier_E862: u32::from_le_bytes([buff[1], buff[5], buff[7], buff[9]]),
            modulator_40: buff[2],
            carrier_40: buff[3],
            feedconn: buff[10],
            finetune: buff[12] as i8,
            notenum: 60,
            noteoffset: 0
        });
    }

    0
}

