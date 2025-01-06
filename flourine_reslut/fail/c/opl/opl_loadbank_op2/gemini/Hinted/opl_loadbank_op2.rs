
#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Clone, Copy)]
enum Op2Flags {
    FixedPitch = 1,
    Unused = 2,
    DoubleVoice = 4,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

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

struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
}

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

struct OplEmu {
    env_counter: u32,
    status: u8,
    timer_running: [u8; 2],
    active_channels: u32,
    modified_channels: u32,
    prepare_count: u32,
    regs: OplEmuRegisters,
    channel: [OplEmuFmChannel; 18],
    operator: [OplEmuFmOperator; 18 * 2],
}

struct Voicealloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

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
    op2_flags: [u16; 256],
}

fn opl_load_op2_voice(timbre: &mut OplTimbre, buff: &[u8]) {
    timbre.modulator_e862 = (buff[3] as u32) << 8 | (buff[2] as u32) << 16 | (buff[1] as u32) << 24 | (buff[0] as u32);
    timbre.carrier_e862 = (buff[10] as u32) << 8 | (buff[9] as u32) << 16 | (buff[8] as u32) << 24 | (buff[7] as u32);
    timbre.modulator_40 = (buff[5] & 0x3f) | ((buff[4] & 0xc0) << 2);
    timbre.carrier_40 = (buff[12] & 0x3f) | ((buff[11] & 0xc0) << 2);
    timbre.feedconn = buff[6];
    timbre.finetune = 0;
    timbre.noteoffset = ((buff[14] as i16) << 8) | (buff[15] as i16);
}

fn opl_loadbank_op2(opl: &mut Opl, data: &[u8], size: usize) -> i32 {
    if size < 8 + 36 * 175 {
        return -3;
    }
    let mut buff = data;
    if buff[0] != b'#' || buff[1] != b'O' || buff[2] != b'P' || buff[3] != b'L' || buff[4] != b'_' || buff[5] != b'I' || buff[6] != b'I' || buff[7] != b'#' {
        return -3;
    }
    buff = &buff[8..];
    opl.is_op2 = 1;
    for i in 0..175 {
        opl.op2_flags[i] = (buff[0] as u16) | ((buff[1] as u16) << 8);
        let finetune = buff[2] as i8;
        let fixednote = buff[3];
        buff = &buff[4..];
        opl_load_op2_voice(&mut opl.opl_gmtimbres[i], buff);
        opl.opl_gmtimbres[i].notenum = fixednote;
        buff = &buff[16..];
        opl_load_op2_voice(&mut opl.opl_gmtimbres_voice2[i], buff);
        opl.opl_gmtimbres_voice2[i].notenum = fixednote;
        opl.opl_gmtimbres_voice2[i].finetune = finetune.wrapping_add(-128);
        buff = &buff[16..];
    }
    0
}
