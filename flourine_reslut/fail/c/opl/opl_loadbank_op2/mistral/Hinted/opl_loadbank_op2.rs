

use std::mem;
use std::convert::TryInto;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u8 = 6;

#[repr(C)]
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
    eg_rate: [u8; OPL_EMU_EG_STATES as usize],
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
    notes2voices: [[[Option<Voicealloc>; 2]; 128]; 16],
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

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_E862 = (buff[3] as u32) << 24
        | (buff[2] as u32) << 16
        | (buff[1] as u32) << 8
        | buff[0] as u32;
    timbre.carrier_E862 = (buff[10] as u32) << 24
        | (buff[9] as u32) << 16
        | (buff[8] as u32) << 8
        | buff[7] as u32;
    timbre.modulator_40 = (buff[5] & 0x3f) | ((buff[4] & 0xc0) as u8);
    timbre.carrier_40 = (buff[12] & 0x3f) | ((buff[11] & 0xc0) as u8);
    timbre.feedconn = buff[6];
    timbre.finetune = buff[4] as i8;
    timbre.notenum = buff[3];
    timbre.noteoffset = (i16::from(buff[15]) << 8) | buff[14] as i16;
}

fn opl_loadbank_op2(opl: &mut Opl, data: &[u8], size: usize) -> i32 {
    if size < 8 + 36 * 175 {
        return -3;
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
        return -3;
    }

    opl.is_op2 = true;

    for i in 0..175 {
        let offset = 4 * (i + 1);
        let flags = (buff[0] as u16) | ((buff[1] as u16) << 8);
        let finetune = buff[2];
        let fixednote = buff[3];

        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], &buff[offset..offset + 16]);
        opl.opl_gmtimbres[i].notenum = fixednote;

        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], &buff[offset + 16..offset + 32]);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune = ((finetune as i32) - 128).wrapping_add(i32::from(opl.opl_gmtimbres_voice2[i].finetune)) as i8;
    }

    let _ = data;
    0
}

