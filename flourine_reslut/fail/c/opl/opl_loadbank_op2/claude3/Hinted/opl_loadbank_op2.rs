

use std::mem;

#[derive(Clone, Copy)]
#[repr(u8)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Clone, Copy)]
struct Op2FlagsT(u8);

impl Op2FlagsT {
    const Op2Fixedpitch: Op2FlagsT = Op2FlagsT(1);
    const Op2Unused: Op2FlagsT = Op2FlagsT(2);
    const Op2Doublevoice: Op2FlagsT = Op2FlagsT(4);

    fn from_bytes(bytes: [u8; 2]) -> Self {
        let value = u16::from_ne_bytes(bytes);
        let value = value as u8;
        Op2FlagsT(value)
    }
}

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

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
    eg_rate: Box<[u8; OplEmuEnvelopeState::OplEmuEgStates as usize]>,
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
    opl3: i32,
    opl_emu: OplEmuT,
    opl_gmtimbres: Box<[OplTimbreT; 256]>,
    opl_gmtimbres_voice2: Box<[OplTimbreT; 256]>,
    is_op2: i32,
    op2_flags: Box<[Op2FlagsT; 256]>,
}

fn opl_load_op2_voice(timbre: &mut OplTimbreT, buff: &[u8]) {
    timbre.modulator_E862 = u32::from(buff[3]); // wave select
    timbre.modulator_E862 = (timbre.modulator_E862 << 8) | u32::from(buff[2]); // sust/release
    timbre.modulator_E862 = (timbre.modulator_E862 << 8) | u32::from(buff[1]); // attack/decay
    timbre.modulator_E862 = (timbre.modulator_E862 << 8) | u32::from(buff[0]); // AM/VIB... flags

    timbre.carrier_E862 = u32::from(buff[10]); // wave select
    timbre.carrier_E862 = (timbre.carrier_E862 << 8) | u32::from(buff[9]); // sust/release
    timbre.carrier_E862 = (timbre.carrier_E862 << 8) | u32::from(buff[8]); // attack/decay
    timbre.carrier_E862 = (timbre.carrier_E862 << 8) | u32::from(buff[7]); // AM/VIB... flags

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
        let op2_flags_bytes = [buff[i * 36 + 8], buff[i * 36 + 9]];
        let op2_flags = Op2FlagsT::from_bytes(op2_flags_bytes);
        let finetune = buff[i * 36 + 10] as i8;
        let fixednote = buff[i * 36 + 11];

        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], &buff[i * 36 + 12..i * 36 + 28]);
        opl.opl_gmtimbres[i].notenum = fixednote;

        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], &buff[i * 36 + 28..i * 36 + 44]);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune = opl.opl_gmtimbres_voice2[i].finetune.wrapping_add(finetune.wrapping_sub(128u8 as i8));

        opl.op2_flags[i] = op2_flags;
    }

    0
}

