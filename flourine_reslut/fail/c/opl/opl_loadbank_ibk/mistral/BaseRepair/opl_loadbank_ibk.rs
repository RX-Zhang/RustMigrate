

use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = 36;
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
    m_cache: Box<OplEmuOpdataCache>,
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
    voices2notes: [Option<Voicealloc>; 18],
    channelprog: [u8; 16],
    opl3: bool,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],
    is_op2: bool,
    op2_flags: [Op2Flags; 256],
}

fn opl_loadbank_internal(
    opl: &mut Opl,
    file: &str,
    offset: i32,
) -> io::Result<()> {
    opl.is_op2 = false;
    let mut buff = [0u8; 16];
    let mut f = File::open(file)?;
    for _ in 0..3204 {
        f.read_exact(&mut buff)?;
    }
    if offset < 0 || (offset as usize) >= 128 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid offset",
        ));
    }
    for i in (0..128).step_by(2) {
        let idx = (offset + i as i32) as usize;
        let mut timbre = &mut opl.opl_gmtimbres[idx];
        timbre.modulator_E862 = (buff[6] as u32)
            | (buff[8] as u32) << 8
            | (buff[4] as u32) << 16
            | (buff[0] as u32) << 24;
        timbre.carrier_E862 = (buff[7] as u32)
            | (buff[9] as u32) << 8
            | (buff[5] as u32) << 16
            | (buff[1] as u32) << 24;
        timbre.modulator_40 = buff[2];
        timbre.carrier_40 = buff[3];
        timbre.feedconn = buff[10];
        timbre.finetune = buff[12] as i8;
        timbre.notenum = 60;
        timbre.noteoffset = 0;
    }
    Ok(())
}

fn opl_loadbank_ibk(opl: &mut Opl, file: &str) -> io::Result<()> {
    let mut instruments = file.to_string();
    let percussion = instruments.split(',').last();
    let instruments = instruments.split(',').next().unwrap();
    let mut f = File::open(instruments)?;
    opl_loadbank_internal(opl, instruments, 0)?;
    if let Some(percussion) = percussion {
        let mut f = File::open(percussion)?;
        opl_loadbank_internal(opl, percussion, 128)?;
    }
    Ok(())
}

