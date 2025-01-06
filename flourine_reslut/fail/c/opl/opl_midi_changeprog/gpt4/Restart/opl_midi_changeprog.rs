
use std::alloc::{alloc_zeroed, Layout};
use std::mem;
use std::ptr::write;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,
}

#[derive(Clone, Copy)]
enum Op2FlagsT {
    Op2Fixedpitch = 1,
    Op2Unused = 2,
    Op2Doublevoice = 4,
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
    eg_rate: [u8; OplEmuEnvelopeState::OplEmuEgStates as usize],
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
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS],
}

struct VoiceallocT {
    priority: u16,
    timbreid: i16,
    channel: i8,
    note: i8,
    voiceindex: u8,
}

struct OplT {
    notes2voices: [[[i8; 2]; 128]; 16],
    channelpitch: [u16; 16],
    channelvol: [u16; 16],
    voices2notes: [VoiceallocT; 18],
    channelprog: [u8; 16],
    opl3: i32,
    opl_emu: Box<OplEmuT>,
    opl_gmtimbres: [OplTimbreT; 256],
    opl_gmtimbres_voice2: [OplTimbreT; 256],
    is_op2: i32,
    op2_flags: [Op2FlagsT; 256],
}

impl OplT {
    fn new(opl3: i32) -> Self {
        unsafe {
            let layout = Layout::new::<OplEmuT>();
            let ptr = alloc_zeroed(layout) as *mut OplEmuT;
            Self {
                notes2voices: [[[0; 2]; 128]; 16],
                channelpitch: [0; 16],
                channelvol: [0; 16],
                voices2notes: mem::zeroed(),
                channelprog: [0; 16],
                opl3,
                opl_emu: Box::from_raw(ptr),
                opl_gmtimbres: mem::zeroed(),
                opl_gmtimbres_voice2: mem::zeroed(),
                is_op2: 0,
                op2_flags: mem::zeroed(),
            }
        }
    }

    fn opl_midi_changeprog(&mut self, channel: i32, program: i32) {
        if channel == 9 {
            return;
        }
        self.channelprog[channel as usize] = program as u8;
    }
}

fn main() {
    let mut opl = OplT::new(1);
    opl.opl_midi_changeprog(1, 64);
}
