

use std::ops::Shl;

#[derive(Copy, Clone)]
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

#[derive(Clone)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; 0x200]>,
    m_waveform: Box<[[u16; 0x400]; 8]>,
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
}

#[derive(Clone)]
struct OplTimbreT {
    modulator_e862: u32,
    carrier_e862: u32,
    modulator_40: u8,
    carrier_40: u8,
    feedconn: u8,
    finetune: i8,
    notenum: u8,
    noteoffset: i16,
}

#[derive(Clone)]
struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: Box<[OplEmuFmChannel; 18]>,
    m_operator: Box<[OplEmuFmOperator; 36]>,
}

#[derive(Clone)]
struct VoiceallocT {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

#[derive(Clone)]
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

fn opl_load_op2_voice(timbre: &mut OplTimbreT, buff: &[u8]) {
    timbre.modulator_e862 = u32::from(buff[3]);
    timbre.modulator_e862 = timbre.modulator_e862.shl(8) | u32::from(buff[2]);
    timbre.modulator_e862 = timbre.modulator_e862.shl(8) | u32::from(buff[1]);
    timbre.modulator_e862 = timbre.modulator_e862.shl(8) | u32::from(buff[0]);

    timbre.carrier_e862 = u32::from(buff[10]);
    timbre.carrier_e862 = timbre.carrier_e862.shl(8) | u32::from(buff[9]);
    timbre.carrier_e862 = timbre.carrier_e862.shl(8) | u32::from(buff[8]);
    timbre.carrier_e862 = timbre.carrier_e862.shl(8) | u32::from(buff[7]);

    timbre.modulator_40 = (buff[5] & 0x3f) | (buff[4] & 0xc0);
    timbre.carrier_40 = (buff[12] & 0x3f) | (buff[11] & 0xc0);

    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from_le_bytes([buff[14], buff[15]]);
}

fn opl_loadbank_op2(opl: &mut OplT, data: &[u8], size: i32) -> i32 {
    if size < 8 + 36 * 175 {
        return -3;
    }

    let buff = data;

    if &buff[0..8] != b"#OPL_II#" {
        return -3;
    }

    opl.is_op2 = 1;

    for i in 0..175 {
        let op2_flags_value = u16::from_le_bytes([buff[i * 36], buff[i * 36 + 1]]);
        opl.op2_flags[i] = match op2_flags_value {
            1 => Op2FlagsT::Op2Fixedpitch,
            2 => Op2FlagsT::Op2Unused,
            4 => Op2FlagsT::Op2Doublevoice,
            _ => Op2FlagsT::Op2Unused,
        };
        let finetune = buff[i * 36 + 2] as i8;
        let fixednote = buff[i * 36 + 3];

        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], &buff[i * 36 + 4..i * 36 + 20]);
        opl.opl_gmtimbres[i].notenum = fixednote;

        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], &buff[i * 36 + 20..i * 36 + 36]);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune += finetune.wrapping_sub(128u8 as i8);
    }

    0
}


