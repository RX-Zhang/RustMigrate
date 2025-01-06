
use std::convert::TryInto;
use std::mem;
use std::ops::{Shr, Shl};
use std::u16;
use std::u8;
use std::mem::MaybeUninit;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(u8)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

#[repr(u8)]
enum Op2Flags {
    OP2_FIXEDPITCH = 1,
    OP2_UNUSED = 2,
    OP2_DOUBLEVOICE = 4,
}

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

#[repr(C)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES],
    eg_shift: u8,
}

#[repr(C)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: &'static mut OplEmuRegisters,
}

#[repr(C)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

#[repr(C)]
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

#[repr(C)]
struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct Opl {
    notes2voices: [[[MaybeUninit<Voicealloc>; 128]; 16]; 2],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [Voicealloc; OPL_EMU_REGISTERS_OPERATORS],
    channelprog: [u8; 16],
    opl3: bool,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    op2_flags: [Op2Flags; 256],
}

fn load_op2_voice(
    timbre: &mut OplTimbre,
    buff: &[u8],
) {
    timbre.modulator_E862 =
        u32::from_le_bytes([buff[0], buff[1], buff[2], buff[3]]);
    timbre.carrier_E862 =
        u32::from_le_bytes([buff[4], buff[5], buff[6], buff[7]]);
    timbre.modulator_40 = buff[8];
    timbre.carrier_40 = buff[9];
    timbre.feedconn = buff[10];
    timbre.finetune = i8::from(buff[11]);
    timbre.notenum = buff[12];
    timbre.noteoffset = i16::from_le_bytes([buff[13], buff[14]]);
}

fn loadbank_op2(
    opl: &mut Opl,
    data: &[u8],
    size: usize,
) -> std::io::Result<i32> {
    if size < 8 + 36 * 175 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid data size",
        ));
    }
    let buff = &data[8..];
    if buff[0] != b'#'
        || buff[1] != b'O'
        || buff[2] != b'P'
        || buff[3] != b'L'
        || buff[4] != b'_'
        || buff[5] != b'I'
        || buff[6] != b'I'
        || buff[7] != b'#'
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid header",
        ));
    }
    buff = &buff[8..];
    opl.is_op2 = true;
    for i in 0..175 {
        let flags =
            Op2Flags::from_u16((buff[0] as u16) | ((buff[1] as u16) << 8)).unwrap();
        let finetune = buff[2];
        let fixednote = buff[3];
        load_op2_voice(&mut opl.opl_gmtimbres[i], buff);
        opl.opl_gmtimbres[i].notenum = fixednote;
        buff = &buff[16..];
        load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], buff);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune += finetune as i8 - 128;
        buff = &buff[16..];
    }
    Ok(0)
}
