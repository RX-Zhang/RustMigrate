
use std::boxed::Box;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;

const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = (1 << OPL_EMU_REGISTERS_CHANNELS) - 1;

const OPL_EMU_REGISTERS_RHYTHM_CHANNEL: u8 = 0xff;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;

const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

const OPL_EMU_REGISTERS_REG_MODE: u16 = 0x04;

const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 5
}

enum OplEmuKeyonType {
    OPL_EMU_KEYON_NORMAL = 0,
    OPL_EMU_KEYON_RHYTHM = 1,
    OPL_EMU_KEYON_CSM = 2
}

enum Op2Flags {
    OP2_FIXEDPITCH = 1,
    OP2_UNUSED = 2,
    OP2_DOUBLEVOICE = 4    
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize],
    eg_shift: u8
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
    m_regs: Box<OplEmuRegisters>   
}

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Box<OplEmuFmOperator>; 4],   
    m_regs: Box<OplEmuRegisters>
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

struct OplEmu {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],   
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS]
}

struct VoiceAlloc {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8   
}   

struct Opl {
    notes2voices: [[[[VoiceAlloc; 2]; 128]; 16]; 2],
    channelpitch: [u16; 16],   
    channelvol: [u16; 16],
    voices2notes: [VoiceAlloc; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: OplEmu,
    opl_gmtimbres: [OplTimbre; 256],
    opl_gmtimbres_voice2: [OplTimbre; 256],   
    is_op2: i32,
    op2_flags: [Op2Flags; 256]
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, op_type: OplEmuKeyonType) {
   // implementation
}

fn opl_emu_fm_channel_keyonoff(fmch: &mut OplEmuFmChannel, states: u32, op_type: OplEmuKeyonType, chnum: u32) {
   // implementation 
}

fn opl_emu_registers_write(regs: &mut OplEmuRegisters, index: u16, data: u8, channel: &mut u32, opmask: &mut u32) -> i32 {
    0
}

fn opl_write(opl: &mut Opl, count: i32, regs: &[u16], data: &[u8]) {
    let emu = &mut opl.opl_emu;
    
    for i in 0..count {
        let regnum = regs[i as usize];
        let value = data[i as usize];

        if regnum == OPL_EMU_REGISTERS_REG_MODE {
            continue;
        }

        emu.m_modified_channels = OPL_EMU_REGISTERS_ALL_CHANNELS;

        let mut keyon_channel = 0;
        let mut keyon_opmask = 0;
        opl_emu_registers_write(&mut emu.m_regs, regnum, value, &mut keyon_channel, &mut keyon_opmask);
    }
}

