

use std::convert::TryInto;

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
    m_cache: OplEmuOpdataCache,  
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,  
}

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

struct Opl {
    notes2voices: [[[(i8, i8); 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [Box<VoiceAlloc>; 18], 
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: Box<OplEmu>,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    op2_flags: [u16; 256],  
}

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,  
    channel: i8,
    note: i8,
    voiceindex: u8,
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    // load modulator
    timbre.modulator_E862 = u32::from(buff[3]);
    timbre.modulator_E862 <<= 8; 
    timbre.modulator_E862 |= u32::from(buff[2]);
    timbre.modulator_E862 <<= 8;
    timbre.modulator_E862 |= u32::from(buff[1]);
    timbre.modulator_E862 <<= 8;
    timbre.modulator_E862 |= u32::from(buff[0]);

    // load carrier
    timbre.carrier_E862 = u32::from(buff[10]);
    timbre.carrier_E862 <<= 8;
    timbre.carrier_E862 |= u32::from(buff[9]);
    timbre.carrier_E862 <<= 8;
    timbre.carrier_E862 |= u32::from(buff[8]);
    timbre.carrier_E862 <<= 8;
    timbre.carrier_E862 |= u32::from(buff[7]);

    // load KSL 
    timbre.modulator_40 = buff[5] & 0x3f | buff[4] & 0xc0; 
    timbre.carrier_40 = buff[12] & 0x3f | buff[11] & 0xc0;

    // feedconn & finetune
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = i16::from(buff[14]) | (i16::from(buff[15]) << 8);
}

fn opl_loadbank_op2(opl: &mut Opl, data: &[u8]) -> Result<(), i32> {
    if data.len() < 8 + 36 * 175 {
        return Err(-3);
    }

    let buff = &data[8..];
    
    opl.is_op2 = true;

    for i in 0..175 {
        // OP2 instrument header 
        opl.op2_flags[i] = u16::from(buff[0]) | ((u16::from(buff[1])) << 8);
        let finetune = buff[2];
        let fixednote = buff[3];
        let buff = &buff[4..];
        
        // first voice
        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], buff); 
        opl.opl_gmtimbres[i].notenum = fixednote;
        let buff = &buff[16..];

        // second voice
        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], buff);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote; 
        opl.opl_gmtimbres_voice2[i]
            .finetune = (opl.opl_gmtimbres_voice2[i].finetune as u8)
            .wrapping_add(finetune) as i8;
        let buff = &buff[16..]; 
    }

    Ok(())
}

