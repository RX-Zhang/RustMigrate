

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(u8)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u8 = 6;

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
    m_env_attenuation: u32,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,
}

impl OplEmuFmOperator {
    fn opl_emu_bitfield(&self, value: u32, start: i32, length: i32) -> u32 {
        ((value >> start) & ((1 << length) - 1)) as u32
    }

    fn opl_emu_attenuation_increment(&self, rate: u32, index: u32) -> u32 {
        let index = index as usize;
        let rate = rate as usize;
        let increment_table: [[u32; 8]; 8] = [
            [0x00000000, 0x00000000, 0x10101010, 0x10101010, 0x20202020, 0x20202020, 0x30303030, 0x30303030],
            [0x30303030, 0x30303030, 0x31313131, 0x31313131, 0x32323232, 0x32323232, 0x33333333, 0x33333333],
            [0x33333333, 0x33333333, 0x34343434, 0x34343434, 0x35353535, 0x35353535, 0x36363636, 0x36363636],
            [0x36363636, 0x36363636, 0x37373737, 0x37373737, 0x38383838, 0x38383838, 0x39393939, 0x39393939],
            [0x39393939, 0x39393939, 0x3a3a3a3a, 0x3a3a3a3a, 0x3b3b3b3b, 0x3b3b3b3b, 0x3c3c3c3c, 0x3c3c3c3c],
            [0x3c3c3c3c, 0x3c3c3c3c, 0x3d3d3d3d, 0x3d3d3d3d, 0x3e3e3e3e, 0x3e3e3e3e, 0x3f3f3f3f, 0x3f3f3f3f],
            [0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f],
            [0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f, 0x3f3f3f3f],
        ];
        increment_table[rate][index]
    }

    fn opl_emu_fm_operator_clock_envelope(&mut self, env_counter: u32) {
        // ...
    }
}

